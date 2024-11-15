use std::error::Error;
use std::io::{Write, Read};
use std::time::Duration;
use std::{thread, vec};
use serialport::SerialPort;
use phf::phf_map;

fn main() -> Result<(), Box<dyn Error>> {
    let poll = "R,33\r\n";
    let initialize_commands = vec!["M,1\r\n", "R,30\r\n", "R,34,FFFF0000\r\n"];

    let mut port = serialport::new("/dev/ttyAMA0", 115200)
        .timeout(Duration::from_millis(5000))
        .open()?;
    println!("Opened serial port successfully.");

    for command in initialize_commands {
        port.write_all(command.as_bytes())?;
        port.flush()?;
    }


    loop {
        port.write_all(poll.as_bytes())?;
        port.flush()?;
        println!("poll");

        let mut read_buf = vec![0; 32];
        println!("{:?}", port.read(&mut read_buf)?);
        let read_len = port.read(&mut read_buf)?;
        println!("{:?}", read_len);
        decoder(&read_buf[..read_len], &mut *port);

        thread::sleep(Duration::from_millis(500));
    }
}

fn decoder(byte_array: &[u8], serial: &mut dyn SerialPort) {
    let bytes = byte_array.chunks(2).map(|chunk| chunk[0] as usize).collect::<Vec<_>>();
    println!("bytes: {:?}", bytes);
    for byte in bytes {
        println!("byte: {:?}", byte);
        execute_command(byte, serial);
    }
}

fn execute_command(byte_command: usize, serial: &mut dyn SerialPort) {
    println!("Executing command: {}", byte_command);
    match BYTE_TABLE.get(&byte_command.to_string().as_str()) {
        Some(ByteCommand::OneRonStacked) => {
            serial.write_all("R,13,0500640001\r\n".as_bytes()).unwrap();
            thread::sleep(Duration::from_millis(100));
            serial.write_all("R,14,01\r\n".as_bytes()).unwrap();
        }
        Some(ByteCommand::FiveRonStacked) => {
            serial.write_all("R,13,0501F40002\r\n".as_bytes()).unwrap();
            thread::sleep(Duration::from_millis(100));
            serial.write_all("R,14,01\r\n".as_bytes()).unwrap();
        }
        Some(ByteCommand::TenRonStacked) => {
            serial.write_all("R,13,0527100003\r\n".as_bytes()).unwrap();
            thread::sleep(Duration::from_millis(100));
            serial.write_all("R,14,01\r\n".as_bytes()).unwrap();
        }
        Some(ByteCommand::FiftyRonStacked) => {
            serial.write_all("R,13,05C3500004\r\n".as_bytes()).unwrap();
            thread::sleep(Duration::from_millis(100));
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