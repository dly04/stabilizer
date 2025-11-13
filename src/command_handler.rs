use smoltcp_nal::smoltcp::socket::tcp::Socket as TcpSocket;
use log::{error, warn};
use core::fmt::Write;
use crate::command_parser::Ipv4Config;
use heapless::Vec;
use crate::command_parser::Command;
use crate::command_parser::ShowCommand;

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

impl Handler {
    pub fn handle_command(
        command: Command,
        socket: &mut TcpSocket,
    ) -> Result<Self, Error> {
        match command {
            // Command::Quit => {},
            Command::Show(ShowCommand::Input) => Handler::show_report(socket, channels),
            // Command::Show(ShowCommand::Pid) => {},
            // Command::Show(ShowCommand::Output) => {},
            // Command::Show(ShowCommand::BParameter) => {}
            // Command::Show(ShowCommand::PostFilter) => {},
            // Command::Show(ShowCommand::Ipv4) => {},
            // Command::OutputPid { channel } => {},
            // Command::OutputPolarity { channel, polarity } => {}
            // Command::Output {
            //     channel,
            //     pin,
            //     value,
            // } => {}
            // Command::Pid {
            //     channel,
            //     parameter,
            //     value,
            // } => {},
            // Command::BParameter {
            //     channel,
            //     parameter,
            //     value,
            // } => {},
            // Command::PostFilter {
            //     channel,
            //     rate: None,
            // } => {},
            // Command::PostFilter {
            //     channel,
            //     rate: Some(rate),
            // } => {},
            // Command::Load { channel } => {},
            // Command::Save { channel } => {},
            // Command::Ipv4(config) => {},
            // Command::Reset => {},
            // Command::Dfu => {},
            // Command::FanSet { fan_pwm } => {},
            // Command::ShowFan => {},
            // Command::FanAuto => {},
            // Command::FanCurve { k_a, k_b, k_c } => {}
            // Command::FanCurveDefaults => {},
            // Command::ShowHWRev => {},
        }
    }

    fn show_report(socket: &mut TcpSocket, channels: &mut Channels) -> Result<Handler, Error> {
        match channels.reports_json() {
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
}