//! Stabilizer network management module
//!
//! # Design
//! The stabilizer network architecture supports numerous layers to permit transmission of
//! telemetry (via MQTT), configuration of run-time settings (via MQTT + Miniconf), and data
//! streaming over raw UDP/TCP sockets. This module encompasses the main processing routines
//! related to Stabilizer networking operations.
use heapless;
use miniconf;

use crate::hardware::{SystemTimer, hal::ethernet};
use platform::{ApplicationMetadata, NetSettings, TelemetryClient};
use stream::{DataStream, FrameGenerator, Target};
use crate::telemetry;
use crate::command_parser::ShowCommand;
use crate::command_handler::Handler;

use core::fmt::Write;
use heapless::String;
use miniconf::{TreeDeserializeOwned, TreeSerialize};
use miniconf_mqtt::minimq;
use smoltcp_nal::smoltcp;
use smoltcp_nal::smoltcp::socket::tcp::Socket as TcpSocket;
use smoltcp_nal::embedded_nal::TcpClientStack;
use smoltcp_nal::NetworkError;
use smoltcp_nal::smoltcp::iface::SocketHandle;

use crate::session::{Session, SessionInput};
use crate::command_parser::Command;
use crate::dual_iir_lib::DualIir;

pub type EthernetPhy = ethernet::phy::LAN8742A<ethernet::EthernetMAC>;

pub type NetworkReference =
    smoltcp_nal::shared::NetworkStackProxy<'static, NetworkStack>;

// Number of TX descriptors in the ethernet descriptor ring.
pub const TX_DESRING_CNT: usize = 4;

// Number of RX descriptors in the ethernet descriptor ring.
pub const RX_DESRING_CNT: usize = 4;

pub type NetworkStack = smoltcp_nal::NetworkStack<
    'static,
    ethernet::EthernetDMA<TX_DESRING_CNT, RX_DESRING_CNT>,
    SystemTimer,
>;

pub type NetworkManager = smoltcp_nal::shared::NetworkManager<
    'static,
    ethernet::EthernetDMA<TX_DESRING_CNT, RX_DESRING_CNT>,
    SystemTimer,
>;

struct MqttStorage {
    telemetry: [u8; 2048],
    settings: [u8; 1536],
}

impl Default for MqttStorage {
    fn default() -> Self {
        Self {
            telemetry: [0u8; 2048],
            settings: [0u8; 1536],
        }
    }
}

pub enum UpdateState {
    SettingsChanged,
    NoChange,
    Updated,
}

pub enum NetworkState {
    SettingsChanged,
    Updated,
    NoChange,
}

const MAX_DEPTH: usize = 16;

/// A structure of Stabilizer's default network users.
pub struct NetworkUsers<S> {
    miniconf: miniconf_mqtt::MqttClient<
        'static,
        S,
        NetworkReference,
        SystemTimer,
        minimq::broker::NamedBroker<NetworkReference>,
        MAX_DEPTH,
    >,
    pub processor: NetworkProcessor,
    stream: DataStream<NetworkReference>,
    generator: Option<FrameGenerator>,
    pub telemetry: TelemetryClient<SystemTimer, NetworkReference>,
}

impl<S> NetworkUsers<S>
where
    S: TreeDeserializeOwned + TreeSerialize,
{
    /// Construct Stabilizer's default network users.
    ///
    /// # Args
    /// * `stack` - The network stack that will be used to share with all network users.
    /// * `phy` - The ethernet PHY connecting the network.
    /// * `clock` - A `SystemTimer` implementing `Clock`.
    /// * `app` - The name of the application.
    /// * `net_settings` - The network-specific settings to use for the application.
    /// * `metadata` - The application metadata
    ///
    /// # Returns
    /// A new struct of network users.
    pub fn new(
        stack: NetworkStack,
        phy: EthernetPhy,
        clock: SystemTimer,
        app: &str,
        net_settings: &NetSettings,
        metadata: &'static ApplicationMetadata,
    ) -> Self {
        let stack_manager =
            cortex_m::singleton!(: NetworkManager = NetworkManager::new(stack))
                .unwrap();

        let mut processor =
            NetworkProcessor::new(stack_manager.acquire_stack(), phy);

            if let Err(e) = processor.start_tcp_server(5678) {
            log::warn!("Failed to start TCP server: {:?}", e);
            } else {
                log::info!("TCP server initialized successfully on port 5678");
            }

        let prefix = cortex_m::singleton!(: String<128> = get_device_prefix(app, &net_settings.id)).unwrap();

        let store =
            cortex_m::singleton!(: MqttStorage = MqttStorage::default())
                .unwrap();

        let named_broker = minimq::broker::NamedBroker::new(
            &net_settings.broker,
            stack_manager.acquire_stack(),
        )
        .unwrap();
        let miniconf = miniconf_mqtt::MqttClient::<_, _, _, _, MAX_DEPTH>::new(
            stack_manager.acquire_stack(),
            prefix.as_str(),
            clock,
            minimq::ConfigBuilder::new(named_broker, &mut store.settings)
                .client_id(&get_client_id(&net_settings.id, "settings"))
                .unwrap(),
        )
        .unwrap();

        let named_broker = minimq::broker::NamedBroker::new(
            &net_settings.broker,
            stack_manager.acquire_stack(),
        )
        .unwrap();
        let mqtt = minimq::Minimq::new(
            stack_manager.acquire_stack(),
            clock,
            minimq::ConfigBuilder::new(named_broker, &mut store.telemetry)
                // The telemetry client doesn't receive any messages except MQTT control packets.
                // As such, we don't need much of the buffer for RX.
                .rx_buffer(minimq::config::BufferConfig::Maximum(100))
                .client_id(&get_client_id(&net_settings.id, "tlm"))
                .unwrap(),
        );

        let telemetry = TelemetryClient::new(mqtt, prefix, metadata);

        let (generator, stream) = stream::setup(stack_manager.acquire_stack());

        NetworkUsers {
            miniconf,
            processor,
            telemetry,
            stream,
            generator: Some(generator),
        }
    }

    /// Enable data streaming.
    ///
    /// # Args
    /// * `format` - A unique u8 code indicating the format of the data.
    pub fn configure_streaming(
        &mut self,
        format: impl Into<u8>,
    ) -> FrameGenerator {
        let mut generator = self.generator.take().unwrap();
        generator.configure(format);
        generator
    }

    /// Direct the stream to the provided remote target.
    ///
    /// # Args
    /// * `remote` - The destination for the streamed data.
    pub fn direct_stream(&mut self, remote: Target) {
        if self.generator.is_none() {
            self.stream.set_remote(remote);
        }
    }

    /// Update and process all of the network users state.
    ///
    /// # Returns
    /// An indication if any of the network users indicated a state change.
    /// The SettingsChanged option contains the path of the settings that changed.
    pub fn update(&mut self, settings: &mut S) -> NetworkState {
        // Update the MQTT clients.
        self.telemetry.update();

        // Update the data stream.
        if self.generator.is_none() {
            self.stream.process();
        }

        // Poll for incoming data.
        let poll_result = match self.processor.update() {
            UpdateState::NoChange => NetworkState::NoChange,
            UpdateState::Updated => NetworkState::Updated,
            UpdateState::SettingsChanged => NetworkState::SettingsChanged,
        };

        let res = self.miniconf.update(settings);
        match res {
            Ok(true) => NetworkState::SettingsChanged,
            _ => poll_result,
        }
    }

    pub fn update_dual_iir(&mut self, settings: &mut DualIir) -> NetworkState {
        // Update the MQTT clients.
        self.telemetry.update();

        // Update the data stream.
        if self.generator.is_none() {
            self.stream.process();
        }

        // Poll for incoming data.
        let poll_result = match self.processor.update_dual_iir(settings) {
            UpdateState::NoChange => NetworkState::NoChange,
            UpdateState::Updated => NetworkState::Updated,
            UpdateState::SettingsChanged => NetworkState::SettingsChanged,
        };

        poll_result
    }
}

/// Get an MQTT client ID for a client.
///
/// # Args
/// * `id` - The base client ID
/// * `mode` - The operating mode of this client. (i.e. tlm, settings)
///
/// # Returns
/// A client ID that may be used for MQTT client identification.
fn get_client_id(id: &str, mode: &str) -> String<64> {
    let mut identifier = String::new();
    write!(&mut identifier, "{id}-{mode}").unwrap();
    identifier
}

/// Get the MQTT prefix of a device.
///
/// # Args
/// * `app` - The name of the application that is executing.
/// * `id` - The MQTT ID of the device.
///
/// # Returns
/// The MQTT prefix used for this device.
fn get_device_prefix(app: &str, id: &str) -> String<128> {
    // Note(unwrap): The mac address + binary name must be short enough to fit into this string. If
    // they are defined too long, this will panic and the device will fail to boot.
    let mut prefix: String<128> = String::new();
    write!(&mut prefix, "dt/sinara/{app}/{id}").unwrap();

    prefix
}

// Task to process network hardware.
//
// # Design
// The network processir is a small taks to regularly process incoming data over ethernet, handle
// the ethernet PHY state, and reset the network as appropriate.

/// Processor for managing network hardware.
pub struct NetworkProcessor {
    stack: NetworkReference,
    phy: EthernetPhy,
    network_was_reset: bool,

    // tcp server part
    tcp_port: u16,
    tcp_server_initialized: bool,
    tcp_server_handle: Option<smoltcp::iface::SocketHandle>,
    tcp_session: Session,
    // share finalized telemetry with tcp server
    pub finalized_telemetry: telemetry::Telemetry,
}

impl NetworkProcessor {
    /// Construct a new network processor.
    ///
    /// # Args
    /// * `stack` - A reference to the shared network stack
    /// * `phy` - The ethernet PHY used for the network.
    ///
    /// # Returns
    /// The newly constructed processor.
    pub fn new(stack: NetworkReference, phy: EthernetPhy) -> Self {
        Self {
            stack,
            phy,
            network_was_reset: false,

            // tcp server part
            tcp_port: 0,
            tcp_server_initialized: false,
            tcp_server_handle: None,
            tcp_session: Session::new(),
            finalized_telemetry: telemetry::Telemetry::default(),
        }
    }

    pub fn start_tcp_server(&mut self, port: u16) -> Result<(), ()> {
        if self.tcp_server_initialized {
            return Ok(());
        }
        
        let result = self.stack.lock(|stack| {
            for (index, (handle, socket)) in stack.sockets.iter_mut().enumerate() {
                if let smoltcp::socket::Socket::Tcp(tcp_socket) = socket {
                    if !tcp_socket.is_active() && !tcp_socket.is_listening() {
                        if tcp_socket.listen(port).is_ok() {
                            log::info!("TCP server started on port {} with handle {:?} at index {}", port, handle, index);
                            self.tcp_port = port;
                            self.tcp_server_initialized = true;
                            self.tcp_server_handle = Some(handle);
                            return Ok(());
                        }
                    }
                }
            }
            Err(())
        });
        
        if result.is_ok() {
            self.tcp_port = port;
            self.tcp_server_initialized = true;
        }
        
        result
    }

    fn restart_tcp_server(&mut self) -> Result<(), ()> {
        if self.tcp_server_initialized {
            self.tcp_server_initialized = false;
            self.start_tcp_server(self.tcp_port)
        } else {
            Ok(())
        }
    }

    pub fn process_tcp_sockets(&mut self, settings: &mut DualIir) -> UpdateState {
        let mut updated = UpdateState::NoChange;
        
        self.stack.lock(|stack| {
            if let Some(server_handle) = self.tcp_server_handle {
                let socket = stack.sockets.get_mut::<smoltcp::socket::tcp::Socket>(server_handle);
                
                if socket.is_active() && !socket.is_listening() {
                    if socket.may_recv() {
                        let mut buffer = [0; 512];
                        match socket.recv_slice(&mut buffer) {
                            Ok(0) => {
                                log::debug!("TCP connection closed by peer");
                                self.tcp_session.reset();
                                updated = UpdateState::Updated;
                            }
                            Ok(len) => {
                                log::info!("TCP received {} bytes", len);
                                match core::str::from_utf8(&buffer[..len]) {
                                    Ok(text) => {
                                        log::info!("message as string: {}", text);
                                    }
                                    Err(e) => {
                                        log::warn!("message contains invalid UTF-8: {}", e);
                                    }
                                }
                                
                                let (bytes_processed, session_input) = self.tcp_session.feed(&buffer[..len]);
                                
                                match session_input {
                                    SessionInput::Nothing => {
                                        log::debug!("Incomplete command, waiting for more data...");
                                    }
                                    SessionInput::Command(command) => {
                                        match Handler::handle_command(
                                            command,
                                            socket,
                                            self.finalized_telemetry.clone(),
                                            settings
                                        ) {
                                            Ok(Handler::NewIPV4(ip)) => {},
                                            Ok(Handler::Handled) => {}
                                            Ok(Handler::SettingsChanged) => { updated = UpdateState::SettingsChanged }
                                            Ok(Handler::CloseSocket) => socket.close(),
                                            Ok(Handler::Reset) => {},
                                            Err(_) => {}
                                        }
                                    }
                                    SessionInput::Error(parser_error) => {
                                        log::warn!("Command parsing error: {:?}", parser_error);
                                        let response = b"{\"error\": \"invalid command format\"}\n";
                                        let _ = socket.send_slice(response);
                                        updated = UpdateState::Updated;
                                    }
                                }
                            }
                            Err(_) => {
                            }
                        }
                    }
                    
                    if !socket.is_open() {
                        log::debug!("TCP connection closed");
                        updated = UpdateState::Updated;
                    }
                }
                // else if !socket.is_active() && !socket.is_listening() {
                //     if self.tcp_server_initialized {
                //         if socket.listen(self.tcp_port).is_ok() {
                //             log::debug!("Re-established TCP listening on port {}", self.tcp_port);
                //             updated = UpdateState::Updated;
                //         }
                //     }
                // }
            }
        });
        
        updated
    }


    /// Handle ethernet link connection status.
    ///
    /// # Note
    /// This may take non-trivial amounts of time to communicate with the PHY. As such, this should
    /// only be called as often as necessary (e.g. once per second or so).
    pub fn handle_link(&mut self) {
        // If the PHY indicates there's no more ethernet link, reset the DHCP server in the network
        // stack.
        let link_up = self.phy.poll_link();
        match (link_up, self.network_was_reset) {
            (true, true) => {
                log::warn!("Network link UP");
                self.network_was_reset = false;
                if let Err(_) = self.restart_tcp_server() {
                    log::warn!("Failed to restart TCP server");
                }
            }
            // Only reset the network stack once per link reconnection. This prevents us from
            // sending an excessive number of DHCP requests.
            (false, false) => {
                log::warn!("Network link DOWN");
                self.network_was_reset = true;
                self.stack.lock(|stack| stack.handle_link_reset());
            }
            _ => {}
        };
    }

    /// Process and update the state of the network.
    ///
    /// # Note
    /// This function should be called regularly before other network tasks to update the state of
    /// all relevant network sockets.
    ///
    /// # Returns
    /// An update state corresponding with any changes in the underlying network.
    pub fn update(&mut self) -> UpdateState {
        let network_updated = match self.stack.lock(|stack| stack.poll()) {
            Ok(true) => UpdateState::Updated,
            Ok(false) => UpdateState::NoChange,
            Err(_) => UpdateState::Updated,
        };

        network_updated
    }
    
    pub fn update_dual_iir(&mut self, settings: &mut DualIir) -> UpdateState {
        let network_updated = match self.stack.lock(|stack| stack.poll()) {
            Ok(true) => UpdateState::Updated,
            Ok(false) => UpdateState::NoChange,
            Err(_) => UpdateState::Updated,
        };

        let tcp_updated = self.process_tcp_sockets(settings);

        match (network_updated, tcp_updated) {
            (UpdateState::SettingsChanged, _) | (_, UpdateState::SettingsChanged) => UpdateState::SettingsChanged,
            (UpdateState::Updated, _) | (_, UpdateState::Updated) => UpdateState::Updated,
            _ => UpdateState::NoChange,
        }
    }
}

