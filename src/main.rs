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

fn decode_input(input: &str) -> Option<ByteCommand> {
    // Remove the prefix "p," from the input
    let cleaned_input = input.trim_start_matches("p,").trim();

    // Extract the relevant part (assuming the pairs are at the end of the string)
    if cleaned_input.len() < 2 {
        return None; // Not enough characters for a valid pair
    }
    let pair = &cleaned_input[cleaned_input.len() - 2..];

    // Convert the pair to a byte
    if let Ok(byte) = u8::from_str_radix(pair, 16) {
        ByteCommand::from_byte(byte)
    } else {
        None
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

    thread::sleep(Duration::from_secs(5));

    let polling_command = "R,33\r\n";

    loop {
        // Send the polling command
        port.write_all(polling_command.as_bytes())?;
        port.flush()?; // Ensure the data is sent
        println!("Sent polling command: {}", polling_command.trim());

        // Read the response from the serial port
        let mut serial_buf: Vec<u8> = vec![0; 32];
        let response = port.read(serial_buf.as_mut_slice())?;

        // Process the received data
        if response > 0 {
            for i in (0..response).step_by(2) {
                if i + 1 < response {
                    let byte_command = serial_buf[i];
                    if let Some(command) = ByteCommand::from_byte(byte_command) {
                        execute_command(command, &mut port);
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(500));
    }
}