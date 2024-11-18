use std::error::Error;
use std::io::{Read, Write};
use std::{str, thread};
use std::time::Duration;
use serialport::SerialPort;

fn execute_command(byte_command: u8, port: &mut dyn SerialPort) -> Result<(), Box<dyn Error>> {
    match byte_command {
        80 => {
            port.write_all(b"R,13,0500640001\r\n")?;
            thread::sleep(Duration::from_millis(100));
            port.write_all(b"R,14,01\r\n")?;
            println!("1 RON Sent");
        }
        81 => {
            port.write_all(b"R,13,0501F40002\r\n")?;
            thread::sleep(Duration::from_millis(100));
            port.write_all(b"R,14,01\r\n")?;
            println!("5 RON Sent");
        }
        82 => {
            port.write_all(b"R,13,0503E80003\r\n")?;
            thread::sleep(Duration::from_millis(100));
            port.write_all(b"R,14,01\r\n")?;
            println!("10 RON Sent");
        }
        83 => {
            port.write_all(b"R,13,0513880004\r\n")?;
            thread::sleep(Duration::from_millis(100));
            port.write_all(b"R,14,01\r\n")?;
            println!("50 RON Sent");
        }
        _ => println!("Invalid byte command: {}", byte_command),
    }
    Ok(())
}

fn decode_input(input: &str, port: &mut dyn SerialPort) -> Result<(), Box<dyn Error>> {
    let cleaned_input = input.trim_start_matches("p,").trim();
    if cleaned_input.len() < 2 {
        println!("Invalid byte input");
        return Ok(());
    }

    if let Ok(byte) = cleaned_input[cleaned_input.len() - 2..].parse::<u8>() {
        execute_command(byte, port)?;
    } else {
        println!("Failed to parse byte command: {}", cleaned_input);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let port_name = "/dev/ttyAMA0"; // Adjust to your serial port
    let mut port = serialport::new(port_name, 115200)
        .timeout(Duration::from_millis(5000))
        .open()?;

    // Send initial commands
    let initial_commands = vec!["M,1\r\n", "R,30\r\n", "R,34,FFFF000032\r\n"];

    for command in initial_commands {
        port.write_all(command.as_bytes())?;
        port.flush()?;
        println!("Sent initial command: {}", command.trim());
    }

    thread::sleep(Duration::from_secs(1));

    let polling_command = "R,33\r\n";

    let mut serial_buf = vec![0; 1024]; // Buffer for serial read

    loop {
        // Send the polling command
        port.write_all(polling_command.as_bytes())?;
        port.flush()?; // Ensure the data is sent
        println!("Sent polling command: {}", polling_command.trim());

        // Read response
        let bytes_read = port.read(serial_buf.as_mut_slice())?;
        let response = str::from_utf8(&serial_buf[..bytes_read])?;

        decode_input(response, &mut *port)?;

        thread::sleep(Duration::from_millis(500));
    }
}
