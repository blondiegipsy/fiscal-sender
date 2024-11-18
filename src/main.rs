use std::error::Error;
use std::io::{Read, Write};
use std::{fs, str, thread};
use std::fs::OpenOptions;
use std::time::{Duration, SystemTime};
use chrono::{Local, NaiveDate, NaiveDateTime};

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
            log_to_file("5 RON sent").unwrap();
        }
        82 => {
            port.write_all(b"R,13,0503E80003\r\n")?;
            thread::sleep(Duration::from_millis(100));
            port.write_all(b"R,14,01\r\n")?;
            log_to_file("10 RON sent").unwrap();
        }
        83 => {
            port.write_all(b"R,13,0513880004\r\n")?;
            thread::sleep(Duration::from_millis(100));
            port.write_all(b"R,14,01\r\n")?;
            log_to_file("50 RON sent").unwrap();
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
    // Get the current date for the log file name
    let date = Local::now().format("%Y-%m-%d").to_string();
    let file_name = format!("logfile-{}.txt", date);

    // Open or create the log file for the day in append mode
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)?;

    // Get the current timestamp for the log entry
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

    // Write the log entry
    writeln!(file, "[{}] Executed: {}", timestamp, command)?;

    // Clean up old log files
    clean_old_logs(60)?;

    Ok(())
}

fn clean_old_logs(retention_days: i64) -> std::io::Result<()> {
    // Get the current date
    let current_date = Local::now().naive_local();
    let log_dir = "."; // Directory containing the logs; adjust if needed

    // Iterate over log files in the directory
    for entry in fs::read_dir(log_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the file name matches the log file pattern
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if file_name.starts_with("logfile-") && file_name.ends_with(".txt") {
                // Extract the date part from the file name
                if let Some(date_str) = file_name.strip_prefix("logfile-").and_then(|n| n.strip_suffix(".txt")) {
                    if let Ok(file_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                        // Calculate the file age
                        let file_age = current_date.signed_duration_since(NaiveDateTime::from(file_date)).num_days();
                        if file_age > retention_days {
                            // Delete the file if older than retention period
                            fs::remove_file(&path)?;
                            println!("Deleted old log file: {}", file_name);
                        }
                    }
                }
            }
        }
    }
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