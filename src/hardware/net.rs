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

use core::fmt::Write;
use heapless::String;
use miniconf::{TreeDeserializeOwned, TreeSerialize};
use miniconf_mqtt::minimq;
// use smoltcp_nal::smoltcp;
use smoltcp_nal::embedded_nal::TcpClientStack;
use smoltcp_nal::NetworkError;
use smoltcp_nal::smoltcp::iface::SocketHandle;

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

        // processor.start_tcp_server(5678);
        processor.connect_to_host([192, 168, 1, 162], 8080);
        processor.log_tcp_connections();

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
        };

        let res = self.miniconf.update(settings);
        match res {
            Ok(true) => NetworkState::SettingsChanged,
            _ => poll_result,
        }
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
        }
    }

    pub fn connect_to_host(&mut self, host_ip: [u8; 4], host_port: u16) {
        let _ = self.stack.lock(|stack| {
            // 尝试获取套接字，失败就返回
            let Ok(mut socket) = stack.socket() else {
                log::warn!("No socket available");
                return;
            };
            
            let addr = core::net::SocketAddr::new(
                core::net::IpAddr::V4(core::net::Ipv4Addr::new(
                    host_ip[0], host_ip[1], host_ip[2], host_ip[3]
                )),
                host_port
            );
            
            // 尝试连接，忽略所有错误
            let _ = stack.connect(&mut socket, addr);
            
            log::info!("Attempted connection to {}.{}.{}.{}:{}", 
                      host_ip[0], host_ip[1], host_ip[2], host_ip[3], host_port);
        });
    }

    // pub fn start_tcp_server(&mut self, port: u16) -> Result<(), ()> {
    //     self.stack.lock(|stack| {
    //         for (handle, socket) in stack.sockets.iter_mut() {
    //             if let smoltcp::socket::Socket::Tcp(tcp_socket) = socket {
    //                 if !tcp_socket.is_active() && !tcp_socket.is_listening() {
    //                     if tcp_socket.listen(port).is_ok() {
    //                         log::info!("TCP server started on port {}", port);
    //                         return Ok(());
    //                     }
    //                 }
    //             }
    //         }
    //         Err(())
    //     })
    // }

    pub fn log_tcp_connections(&mut self) {
        self.stack.lock(|stack| {
            log::info!("=== TCP Socket States ===");
            
            let mut stats = [0, 0, 0]; // [listening, outbound, inactive]
            
            for (handle, socket) in stack.sockets.iter() {
                if let smoltcp_nal::smoltcp::socket::Socket::Tcp(tcp_socket) = socket {
                    if tcp_socket.is_listening() {
                        stats[0] += 1;
                        if let Some(local) = tcp_socket.local_endpoint() {
                            log::info!("LISTENING - Port: {}, Handle: {:?}", local.port, handle);
                        }
                    } else if tcp_socket.is_active() {
                        stats[1] += 1;
                        let local_port = tcp_socket.local_endpoint().map(|ep| ep.port).unwrap_or(0);
                        if let Some(remote) = tcp_socket.remote_endpoint() {
                            log::info!("OUTBOUND - {} -> {}:{}", local_port, remote.addr, remote.port);
                        }
                    } else {
                        stats[2] += 1;
                        log::info!("INACTIVE - Handle: {:?}", handle);
                    }
                }
            }
            
            log::info!("Summary: Listen={}, Outbound={}, Inactive={}", stats[0], stats[1], stats[2]);
            log::info!("============================");
        });
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
                // self.start_tcp_server(5678);
                self.connect_to_host([192, 168, 1, 162], 8080);
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
        match self.stack.lock(|stack| stack.poll()) {
            Ok(true) => UpdateState::Updated,
            Ok(false) => UpdateState::NoChange,
            Err(_) => UpdateState::Updated,
        }
    }
}

/*
python3 -c "
import socket, time
s = socket.socket()
s.bind(('0.0.0.0', 8080))
s.listen(1)
print('🚀 TCP Server started on port 8080')
print('📡 Waiting for device connection...')
client, addr = s.accept()
print(f'✅ Device connected: {addr}')
client.send(b'Welcome from host!\\n')
while True:
    try:
        data = client.recv(1024)
        if data:
            print('Received:', data.decode().strip())
            client.send(b'Echo: ' + data)
        else:
            break
    except:
        break
print('❌ Connection closed')
client.close()
"
*/
