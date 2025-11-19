use smoltcp_nal::smoltcp::socket::tcp::Socket as TcpSocket;
use log::{error, warn};
use core::fmt::Write;
use crate::command_parser::Ipv4Config;
use heapless::Vec;
use crate::command_parser::Command;
use crate::command_parser::ShowCommand;
use crate::telemetry;
use serde::{Serialize, Serializer};
use serde_json_core;

#[derive(Debug, Clone, PartialEq)]
pub enum Handler {
    Handled,
    CloseSocket,
    NewIPV4(Ipv4Config),
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
    ) -> Result<Self, Error> {
        match command {
            Command::Show(ShowCommand::Input) => Handler::show_report(socket, telemetry),
            Command::Show(ShowCommand::Ipv4) => Handler::show_ipv4(socket),
            _ => todo!(),
        }
    }

    fn show_report(socket: &mut TcpSocket, telemetry: telemetry::Telemetry) -> Result<Handler, Error> {
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

    fn show_ipv4(socket: &mut TcpSocket) -> Result<Handler, Error> {
        send_line(socket, b"IPv4: 192.168.1.100/24, GW=192.168.1.1");
        Ok(Handler::Handled)
    }
}