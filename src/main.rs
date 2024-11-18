use std::error::Error;
use std::io::{Write};
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum ByteCommand {
    OneRonStacked,
    FiveRonStacked,
    TenRonStacked,
    FiftyRonStacked,
}

impl ByteCommand {
    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x80 => Some(ByteCommand::OneRonStacked),
            0x81 => Some(ByteCommand::FiveRonStacked),
            0x82 => Some(ByteCommand::TenRonStacked),
            0x83 => Some(ByteCommand::FiftyRonStacked),
            _ => None,
        }
    }
}

fn execute_command(byte_command: ByteCommand, port: &mut dyn Write) {
    match byte_command {
        ByteCommand::OneRonStacked => {
            port.write_all(b"R,13,0500640001\r\n").unwrap();
            thread::sleep(Duration::from_millis(100));
            port.write_all(b"R,14,01\r\n").unwrap();
        }
        ByteCommand::FiveRonStacked => {
            port.write_all(b"R,13,0501F40002\r\n").unwrap();
            thread::sleep(Duration::from_millis(100));
            port.write_all(b"R,14,01\r\n").unwrap();
        }
        ByteCommand::TenRonStacked => {
            port.write_all(b"R,13,0527100003\r\n").unwrap();
            thread::sleep(Duration::from_millis(100));
            port.write_all(b"R,14,01\r\n").unwrap();
        }
        ByteCommand::FiftyRonStacked => {
            port.write_all(b"R,13,05C3500004\r\n").unwrap();
            thread::sleep(Duration::from_millis(100));
            port.write_all(b"R,14,01\r\n").unwrap();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let port_name = "/dev/ttyAMA0"; // Change this to your device
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
        let n = port.read(serial_buf.as_mut_slice())?;

        // Process the received data
        if n > 0 {
            for i in (0..n).step_by(2) {
                if i + 1 < n {
                    let byte_command = serial_buf[i];
                    if let Some(command) = ByteCommand::from_byte(byte_command) {
                        execute_command(command, &mut port);
                    } else {
                        println!("Unknown command byte received: {}", byte_command);
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(500));
    }
}