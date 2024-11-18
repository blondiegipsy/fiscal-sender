use std::error::Error;
use std::io::{Read, Write};
use std::{str, thread};
use std::fs::OpenOptions;
use std::time::{Duration, SystemTime};
use serialport::SerialPort;

fn execute_command(byte_command: u8, port: &mut dyn SerialPort) -> Result<(), Box<dyn Error>> {

    match byte_command {

        80 => {
            port.write_all(b"R,13,0500640001\r\n")?;
            thread::sleep(Duration::from_millis(100));
            port.write_all(b"R,14,01\r\n")?;
            log_to_file("1 RON sent").unwrap();
        }
        81 => {
            port.write_all(b"R,13,0501F40002\r\n")?;
            thread::sleep(Duration::from_millis(100));
            port.write_all(b"R,14,01\r\n")?;
            log_to_file("1 RON sent").unwrap();
        }
        82 => {
            port.write_all(b"R,13,0503E80003\r\n")?;
            thread::sleep(Duration::from_millis(100));
            port.write_all(b"R,14,01\r\n")?;
            log_to_file("1 RON sent").unwrap();
        }
        83 => {
            port.write_all(b"R,13,0513880004\r\n")?;
            thread::sleep(Duration::from_millis(100));
            port.write_all(b"R,14,01\r\n")?;
            log_to_file("1 RON sent").unwrap();
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

fn log_to_file(command: &str) -> std::io::Result<()> {
    // Open or create a log file in append mode
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logfile.txt")?;

    // Get the current timestamp
    let timestamp = SystemTime::now();
    // Write the log entry
    writeln!(file, "[{:?}] Executed: {:?}", timestamp, command)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let port_name = "/dev/ttyAMA0";
    let mut port = serialport::new(port_name, 115200)
        .timeout(Duration::from_millis(5000))
        .open()?;

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
        port.write_all(polling_command.as_bytes())?;
        port.flush()?; // Ensure the data is sent
        println!("Sent polling command: {}", polling_command.trim());

        let bytes_read = port.read(serial_buf.as_mut_slice())?;
        let response = str::from_utf8(&serial_buf[..bytes_read])?;

        decode_input(response, &mut *port)?;

        thread::sleep(Duration::from_millis(500));
    }
}