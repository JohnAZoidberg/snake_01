use std::thread;
use std::time::Duration;

use chrono::Local;
use rand::prelude::*;
use serialport::{SerialPort, SerialPortInfo, SerialPortType};
const FWK_MAGIC: &[u8] = &[0x32, 0xAC];

pub const FRAMEWORK_VID: u16 = 0x32AC;
pub const LED_MATRIX_PID: u16 = 0x0020;

// TODO: Use a shared enum with the firmware code
#[derive(Clone, Copy)]
#[repr(u8)]
enum Command {
    _Brightness = 0x00,
    _Pattern = 0x01,
    _Bootloader = 0x02,
    _Sleeping = 0x03,
    _Animate = 0x04,
    _Panic = 0x05,
    DisplayBwImage = 0x06,
    SendCol = 0x07,
    CommitCols = 0x08,
    _B1Reserved = 0x09,
    _StartGame = 0x10,
    _GameControl = 0x11,
    _GameStatus = 0x12,
    _SetColor = 0x13,
    _DisplayOn = 0x14,
    _InvertScreen = 0x15,
    _SetPixelColumn = 0x16,
    _FlushFramebuffer = 0x17,
    _ClearRam = 0x18,
    _ScreenSaver = 0x19,
    _Fps = 0x1A,
    _PowerMode = 0x1B,
    _AnimationPeriod = 0x1C,
    _PwmFreq = 0x1E,
    _DebugMode = 0x1F,
    _Version = 0x20,
}

const SERIAL_TIMEOUT: Duration = Duration::from_millis(20);

fn match_serialdevs(ports: &[SerialPortInfo], requested: &Option<String>, pid: Option<u16>) -> Vec<String> {
    if let Some(requested) = requested {
        for p in ports {
            if requested == &p.port_name {
                return vec![p.port_name.clone()];
            }
        }
        vec![]
    } else {
        let mut compatible_devs = vec![];
        let pids = if let Some(pid) = pid {
            vec![pid]
        } else {
            // By default accept any type
            vec![LED_MATRIX_PID, 0x22, 0xFF]
        };
        // Find all supported Framework devices
        for p in ports {
            if let SerialPortType::UsbPort(usbinfo) = &p.port_type {
                if usbinfo.vid == FRAMEWORK_VID && pids.contains(&usbinfo.pid) {
                    compatible_devs.push(p.port_name.clone());
                }
            }
        }
        compatible_devs
    }
}

pub fn find_serialdevs(wait_for_device: bool) -> (Vec<String>, bool) {
    let mut serialdevs: Vec<String>;
    let mut waited = false;
    loop {
        let ports = serialport::available_ports().expect("No ports found!");
        serialdevs = match_serialdevs(&ports, &None, Some(LED_MATRIX_PID));
        println!("{:?}", serialdevs);
        if serialdevs.is_empty() {
            if wait_for_device {
                // Waited at least once, that means the device was not present
                // when the program started
                waited = true;

                // Try again after short wait
                thread::sleep(Duration::from_millis(100));
                continue;
            } else {
                return (vec![], waited);
            }
        } else {
            break;
        }
    }
    (serialdevs, waited)
}

fn simple_cmd(serialdev: &str, command: Command, args: &[u8]) {
    let port_result = serialport::new(serialdev, 115_200).timeout(SERIAL_TIMEOUT).open();

    match port_result {
        Ok(mut port) => simple_cmd_port(&mut port, command, args),
        Err(error) => match error.kind {
            serialport::ErrorKind::Io(std::io::ErrorKind::PermissionDenied) => panic!("Permission denied, couldn't access inputmodule serialport. Ensure that you have permission, for example using a udev rule or sudo."),
            other_error => panic!("Couldn't open port: {:?}", other_error)
        }
    };
}

fn simple_cmd_port(port: &mut Box<dyn SerialPort>, command: Command, args: &[u8]) {
    let mut buffer: [u8; 64] = [0; 64];
    buffer[..2].copy_from_slice(FWK_MAGIC);
    buffer[2] = command as u8;
    buffer[3..3 + args.len()].copy_from_slice(args);
    port.write_all(&buffer[..3 + args.len()]).expect("Write failed!");
}

/// Stage greyscale values for a single column. Must be committed with commit_cols()
pub fn send_col(port: &mut Box<dyn SerialPort>, x: u8, vals: &[u8]) {
    let mut buffer: [u8; 64] = [0; 64];
    buffer[0] = x;
    buffer[1..vals.len() + 1].copy_from_slice(vals);
    simple_cmd_port(port, Command::SendCol, &buffer[0..vals.len() + 1]);
}

/// Commit the changes from sending individual cols with send_col(), displaying the matrix.
/// This makes sure that the matrix isn't partially updated.
pub fn commit_cols(port: &mut Box<dyn SerialPort>) {
    simple_cmd_port(port, Command::CommitCols, &[]);
}

/// Show a black/white matrix
/// Send everything in a single command
pub fn render_matrix(serialdev: &str, matrix: &[[u8; 34]; 9]) {
    // One bit for each LED, on or off
    // 39 = ceil(34 * 9 / 8)
    let mut vals: [u8; 39] = [0x00; 39];

    for x in 0..9 {
        for y in 0..34 {
            let i = x + 9 * y;
            if matrix[x][y] == 0xFF {
                vals[i / 8] |= 1 << (i % 8);
            }
        }
    }

    simple_cmd(serialdev, Command::DisplayBwImage, &vals);
}

/// Show a black/white matrix
/// Send everything in a single command
pub fn render_matrix_port(port: &mut Box<dyn SerialPort>, matrix: &[[u8; 34]; 9]) {
    // One bit for each LED, on or off
    // 39 = ceil(34 * 9 / 8)
    let mut vals: [u8; 39] = [0x00; 39];

    for x in 0..9 {
        for y in 0..34 {
            let i = x + 9 * y;
            if matrix[x][y] == 0xFF {
                vals[i / 8] |= 1 << (i % 8);
            }
        }
    }

    simple_cmd_port(port, Command::DisplayBwImage, &vals);
}

