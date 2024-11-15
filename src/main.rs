use std::io::{Write, Read};
use std::time::Duration;
use std::{vec};
use std::error::Error;
use serialport::SerialPort;
use phf::phf_map;

fn main() -> Result<(), Box<dyn Error>> {
    let port_name = "/dev/ttyAMA0";
    let mut port = serialport::new(port_name, 115200)
        .timeout(Duration::from_millis(5000))
        .open()?;

    // Send initial commands
    let initial_commands = vec![",R,30\r\n", "R,34,FFFF000032\n"];

    for command in initial_commands {
        port.write_all(command.as_bytes())?;
        port.flush()?;
        println!("Sent initial command: {}", command.trim());
    }

    // Polling every second with R,33
    let polling_command = "R,33\r\n";

    loop {
        // Send the polling command
        port.write_all(polling_command.as_bytes())?;
        port.flush()?; // Ensure the data is sent
        println!("Sent polling command: {}", polling_command.trim());

        // Read the response from the serial port
        let mut serial_buf: Vec<u8> = vec![0; 32];
        match port.read(&mut serial_buf) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    // Check if the response contains "p,03"
                    if let Ok(decoded_data) = String::from_utf8(serial_buf[..bytes_read].to_vec()) {
                        println!("Received ASCII response: {}", decoded_data);
                        execute_command(&decoded_data, port.as_mut());
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                println!("No response received within timeout.");
            }
            Err(e) => {
                eprintln!("Error reading from serial port: {:?}", e);
                return Err(Box::new(e));
            }
        }

        std::thread::sleep(Duration::from_millis(500));
    }
}

fn execute_command(byte_command: &str, serial: &mut dyn SerialPort) {
    println!("Executing command: {}", byte_command);
    match BYTE_TABLE.get(byte_command) {
        Some(ByteCommand::OneRonStacked) => {
            serial.write_all("R,13,0500640001\r\n".as_bytes()).unwrap();
            std::thread::sleep(Duration::from_millis(100));
            serial.write_all("R,14,01\r\n".as_bytes()).unwrap();
        }
        Some(ByteCommand::FiveRonStacked) => {
            serial.write_all("R,13,0501F40002\r\n".as_bytes()).unwrap();
            std::thread::sleep(Duration::from_millis(100));
            serial.write_all("R,14,01\r\n".as_bytes()).unwrap();
        }
        Some(ByteCommand::TenRonStacked) => {
            serial.write_all("R,13,0527100003\r\n".as_bytes()).unwrap();
            std::thread::sleep(Duration::from_millis(100));
            serial.write_all("R,14,01\r\n".as_bytes()).unwrap();
        }
        Some(ByteCommand::FiftyRonStacked) => {
            serial.write_all("R,13,05C3500004\r\n".as_bytes()).unwrap();
            std::thread::sleep(Duration::from_millis(100));
            serial.write_all("R,14,01\r\n".as_bytes()).unwrap();
        }
        _ => {}
    }
}

#[derive(PartialEq, Eq, Hash)]
enum ByteCommand {
    OneRonStacked,
    FiveRonStacked,
    TenRonStacked,
    FiftyRonStacked,
}

static BYTE_TABLE: phf::Map<&'static str, ByteCommand> = phf_map! {
    "80" => ByteCommand::OneRonStacked,
    "81" => ByteCommand::FiveRonStacked,
    "82" => ByteCommand::TenRonStacked,
    "83" => ByteCommand::FiftyRonStacked
};