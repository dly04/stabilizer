use smoltcp_nal::smoltcp::socket::tcp::Socket as TcpSocket;
use log::{error, info, warn};
use core::fmt::Write;
use heapless::Vec;
use crate::telemetry;
use serde::{Serialize, Serializer};
use serde_json_core;

use crate::dual_iir_lib::DualIir;
use miniconf::Leaf;

use crate::{
    convert,
    command_parser,
    command_parser::{Command, ShowCommand}
};

#[derive(Debug, Clone, PartialEq)]
pub enum Handler {
    Handled,
    SettingsChanged,
    CloseSocket,
    Reset,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Report,
    Flash,
}

pub type JsonBuffer = Vec<u8, 1024>;

fn send_line(socket: &mut TcpSocket, data: &[u8]) -> bool {
    let send_free = socket.send_capacity() - socket.send_queue();
    if data.len() > send_free + 1 {
        // Not enough buffer space, skip report for now,
        // instead of sending incomplete line
        warn!(
            "TCP socket has only {}/{} needed {}",
            send_free + 1,
            socket.send_capacity(),
            data.len(),
        );
    } else {
        match socket.send_slice(data) {
            Ok(sent) if sent == data.len() => {
                let _ = socket.send_slice(b"\n");
                // success
                return true;
            }
            Ok(sent) => warn!("sent only {}/{} bytes", sent, data.len()),
            Err(e) => error!("error sending line: {:?}", e),
        }
    }
    // not success
    false
}

pub fn reports_json(telemetry: & telemetry::Telemetry) -> Result<JsonBuffer, serde_json_core::ser::Error> {
    let mut reports = Vec::<_, 1>::new();
    let _ = reports.push(telemetry);
    serde_json_core::to_vec(&reports)
}

impl Handler {
    pub fn handle_command(
        command: Command,
        socket: &mut TcpSocket,
        telemetry: telemetry::Telemetry,
        settings: &mut DualIir,
    ) -> Result<Self, Error> {
        match command {
            Command::Show(ShowCommand::Input) => Handler::show_report(socket, telemetry),
            Command::PounderFrequency { frequency } => Handler::change_pounder_frequency(socket, settings, frequency),
            Command::Gain {channel, gain} => Handler::gain(socket, settings, channel, gain),
            Command::BiquadBa(biquad_ba_data) => Handler::biquad_ba(socket, settings, biquad_ba_data),
            Command::BiquadRaw(biquad_raw_data) => Handler::biquad_raw(socket),
            Command::BiquadPid(biquad_pid_data) => Handler::biquad_pid(socket),
            Command::BiquadFilter(biquad_filter_data) => Handler::biquad_filter(socket),
            Command::Run { channel, run } => Handler::run(socket),
            Command::Source(source_data) => Handler::source(socket),
            Command::Trigger => Handler::trigger(socket),
            Command::TelemetryPeriod(telemetry_period) => Handler::telemetry_period(socket),
            Command::Stream{ ip, port } => Handler::stream(socket),
            Command::PounderClock(pounder_clock_data) => Handler::pounder_clock(socket),
            Command::PounderChannel(pounder_channel_data) => Handler::pounder_channel(socket),
            _ => todo!(),
        }
    }

    fn show_report(socket: &mut TcpSocket, telemetry: telemetry::Telemetry) -> Result<Handler, Error> {
        info!("handling report");
        match reports_json(&telemetry) {
            Ok(buf) => {
                send_line(socket, &buf[..]);
            }
            Err(e) => {
                error!("unable to serialize report: {:?}", e);
                let _ = writeln!(socket, "{{\"error\":\"{:?}\"}}", e);
                return Err(Error::Report);
            }
        }
        Ok(Handler::Handled)
    }

    fn change_pounder_frequency(socket: &mut TcpSocket, settings: &mut DualIir, frequency: u32) -> Result<Handler, Error> {
        if let Some(pounder) = &mut settings.pounder {
        pounder.out_channel[0].dds.frequency = Leaf(frequency as f32);
        pounder.out_channel[0].dds.amplitude = Leaf(1.0);
        pounder.out_channel[0].attenuation = Leaf(16.0);
        send_line(socket, b"{}");
        Ok(Handler::SettingsChanged)
        } else {
            send_line(socket, b"{}");
            Ok(Handler::Handled)
        }
    }

    fn gain(socket: &mut TcpSocket, settings: &mut DualIir, channel: usize, gain: convert::Gain) -> Result<Handler, Error> {
        info!("handling gain");
        settings.ch[channel].gain = gain;
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn biquad_ba(socket: &mut TcpSocket, settings: &mut DualIir, biquad_ba_data: command_parser::BiquadBaData) -> Result<Handler, Error> {
        info!("handling biquad_ba");
        match biquad_ba_data.field {
            command_parser::BiquadBaField::Ba => {
                settings.ch[channel].biquad[0].repr.
            } 
        }
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn biquad_raw(socket: &mut TcpSocket) -> Result<Handler, Error> {
        info!("handling biquad_raw");
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn biquad_pid(socket: &mut TcpSocket) -> Result<Handler, Error> {
        info!("handling biquad_pid");
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn biquad_filter(socket: &mut TcpSocket) -> Result<Handler, Error> {
        info!("handling biquad_filter");
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn run(socket: &mut TcpSocket) -> Result<Handler, Error> {
        info!("handling run");
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn source(socket: &mut TcpSocket) -> Result<Handler, Error> {
        info!("handling source");
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn trigger(socket: &mut TcpSocket) -> Result<Handler, Error> {
        info!("handling trigger");
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn telemetry_period(socket: &mut TcpSocket) -> Result<Handler, Error> {
        info!("handling telemetry_period");
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn stream(socket: &mut TcpSocket) -> Result<Handler, Error> {
        info!("handling stream");
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn pounder_clock(socket: &mut TcpSocket) -> Result<Handler, Error> {
        info!("handling pounder_clock");
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn pounder_channel(socket: &mut TcpSocket) -> Result<Handler, Error> {
        info!("handling pounder_channel");
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }
}