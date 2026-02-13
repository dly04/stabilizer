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
use idsp::iir::BiquadRepr;
use crate::dual_iir_lib::Run;

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
            Command::BiquadRaw(biquad_raw_data) => Handler::biquad_raw(socket, settings, biquad_raw_data),
            Command::BiquadPid(biquad_pid_data) => Handler::biquad_pid(socket, settings, biquad_pid_data),
            Command::BiquadFilter(biquad_filter_data) => todo!(),
            Command::Run { channel, run } => Handler::run(socket, settings, channel, run),
            Command::Source(source_data) => Handler::source(socket, settings, source_data),
            Command::Trigger => Handler::trigger(socket),
            Command::TelemetryPeriod(telemetry_period) => Handler::telemetry_period(socket),
            Command::Stream{ ip, port } => Handler::stream(socket),
            Command::PounderClock(pounder_clock_data) => Handler::pounder_clock(socket),
            Command::PounderChannel(pounder_channel_data) => Handler::pounder_channel(socket),
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

    fn biquad_ba(socket: &mut TcpSocket, settings: &mut DualIir, cmd: command_parser::BiquadBaData) -> Result<Handler, Error> {
        info!("handling biquad_ba");
        let channel = cmd.channel;
        match &mut settings.ch[channel].biquad[0].repr {
            BiquadRepr::Ba(data) => {
                match &cmd.field {
                    command_parser::BiquadBaField::Ba => {
                        data.ba = [[cmd.ba[0], cmd.ba[1], cmd.ba[2]], [cmd.ba[3], cmd.ba[4], cmd.ba[5]]];
                    }
                    command_parser::BiquadBaField::U => {
                        data.u = cmd.u
                    }
                    command_parser::BiquadBaField::Min => {
                        data.min = cmd.min
                    }
                    command_parser::BiquadBaField::Max => {
                        data.max = cmd.max
                    }
                }
            }
            _ => {
                let ba = idsp::iir::Ba::<f32> {
                    ba: [[cmd.ba[0], cmd.ba[1], cmd.ba[2]], [cmd.ba[3], cmd.ba[4], cmd.ba[5]]],
                    u: cmd.u,
                    min: cmd.min,
                    max: cmd.max
                };
                settings.ch[channel].biquad[0].repr = BiquadRepr::Ba(ba);
            }
        }
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn biquad_raw(socket: &mut TcpSocket, settings: &mut DualIir, cmd: command_parser::BiquadRawData) -> Result<Handler, Error> {
        info!("handling biquad_raw");
        let channel = cmd.channel;
        match &mut settings.ch[channel].biquad[0].repr {
            BiquadRepr::Raw(data) => {
                match cmd.field {
                    command_parser::BiquadRawField::Ba => {
                        let ba_mut = data.ba_mut();
                        *ba_mut = [cmd.ba[0], cmd.ba[1], cmd.ba[2], cmd.ba[3], cmd.ba[4]]
                    },
                    command_parser::BiquadRawField::U => {
                        data.set_u(cmd.u)
                    },
                    command_parser::BiquadRawField::Min => {
                        data.set_min(cmd.min)
                    },
                    command_parser::BiquadRawField::Max => {
                        data.set_max(cmd.max)
                    }
                }
            }
            _ => {
                let mut data = idsp::iir::Biquad::<f32>::default();
                match cmd.field {
                    command_parser::BiquadRawField::Ba => {
                        let ba_mut = data.ba_mut();
                        *ba_mut = [cmd.ba[0], cmd.ba[1], cmd.ba[2], cmd.ba[3], cmd.ba[4]]
                    },
                    command_parser::BiquadRawField::U => {
                        data.set_u(cmd.u)
                    },
                    command_parser::BiquadRawField::Min => {
                        data.set_min(cmd.min)
                    },
                    command_parser::BiquadRawField::Max => {
                        data.set_max(cmd.max)
                    }
                };
                settings.ch[channel].biquad[0].repr = BiquadRepr::Raw(data)
            }
        }
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn biquad_pid(socket: &mut TcpSocket, settings: &mut DualIir, cmd: command_parser::BiquadPidData) -> Result<Handler, Error> {
        info!("handling biquad_pid");
        let channel = cmd.channel;
        match &mut settings.ch[channel].biquad[0].repr {
            BiquadRepr::Pid(data) => {
                match cmd.field {
                    command_parser::BiquadPidField::Order => {
                        data.order = cmd.order
                    }
                    command_parser::BiquadPidField::Gain => {
                        data.gain.value = [cmd.gain.i2, cmd.gain.i, cmd.gain.p, cmd.gain.d, cmd.gain.d2]
                    }
                    command_parser::BiquadPidField::Limit => {
                        data.limit.value = [cmd.limit.i2, cmd.gain.i, cmd.gain.p, cmd.gain.d, cmd.gain.d2]
                    }
                    command_parser::BiquadPidField::Setpoint => {
                        data.setpoint = cmd.setpoint
                    }
                    command_parser::BiquadPidField::Min => {
                        data.min = cmd.min
                    }
                    command_parser::BiquadPidField::Max => {
                        data.max = cmd.max
                    }
                }
            }
            _ => {
                let mut data = idsp::iir::Pid::<f32> {
                    order: cmd.order,
                    gain: idsp::iir::Gain::<f32>::default(),
                    limit: idsp::iir::Gain::<f32>::default(),
                    setpoint: cmd.setpoint,
                    min: cmd.min,
                    max: cmd.max
                };
                data.gain.value = [cmd.gain.i2, cmd.gain.i, cmd.gain.p, cmd.gain.d, cmd.gain.d2];
                data.limit.value = [cmd.limit.i2, cmd.limit.i, cmd.limit.p, cmd.limit.d, cmd.limit.d2];
                settings.ch[channel].biquad[0].repr = BiquadRepr::Pid(data)
            }
        }
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn biquad_filter(socket: &mut TcpSocket) -> Result<Handler, Error> {
        info!("handling biquad_filter");
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn run(socket: &mut TcpSocket, settings: &mut DualIir, channel:usize, run: Run) -> Result<Handler, Error> {
        info!("handling run");
        settings.ch[channel].run = run;
        send_line(socket, b"{}");
        Ok(Handler::Handled)
    }

    fn source(socket: &mut TcpSocket, settings: &mut DualIir, cmd: command_parser::SourceData) -> Result<Handler, Error> {
        info!("handling source");
        let channel = cmd.channel;
        // match cmd.field {
        //     command_parser::SourceField::Signal => {}
        // }
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