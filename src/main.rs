use std::error::Error;
use std::io::{self, Write, Read};
use std::time::Duration;
use std::{thread, vec};
use serialport::SerialPort;
use phf::phf_map;

fn main() -> Result<(), Box<dyn Error>> {
    let poll = "R,33\r\n";
    let initialize_commands = vec!["M,1\r\n", "R,30\r\n","R,34,FFFF0000\r\n"];

    let mut port = serialport::new("dev/ttyAMA0", 115200)
        .timeout(Duration::from_millis(5000))
        .open()?;
    println!("Opened serial port successfully.");

    for command in initialize_commands {
        port.write_all(command.as_bytes())?;
        port.flush()?;
    }

    let mut response_buf: Vec<u8> = vec![0; 32];

    loop {
        port.write_all(poll.as_bytes())?;
        port.flush()?;

        let read_buf = port.read(response_buf.as_mut_slice())?;
        decoder(&read_buf, &port);


        thread::sleep(Duration::from_millis(500));
    }
}

fn decoder(byte_array: &usize, serial: &Box<dyn SerialPort>) {
    let bytes = byte_array.as_bytes().chunks(2).map(|chunk| chunk[0] as usize).collect();

    for byte in bytes {
        execute_command(byte, &serial);
    }
}

fn execute_command(byte_command: ByteCommand, mut serial: Box<dyn SerialPort>) {
    match byte_command {
        ByteCommand::OneRonStacked => {
            serial.write_all("R,13,0500640001\r\n".as_bytes()).unwrap();
            thread::sleep(Duration::from_millis(100));
            serial.write_all("R,14,01\r\n".as_bytes()).unwrap();
        }
        ByteCommand::FiveRonStacked => {
            serial.write_all("R,13,0501F40002\r\n".as_bytes()).unwrap();
            thread::sleep(Duration::from_millis(100));
            serial.write_all("R,14,01\r\n".as_bytes()).unwrap();

        }
        ByteCommand::TenRonStacked => {
            serial.write_all("R,13,0527100003\r\n".as_bytes()).unwrap();
            thread::sleep(Duration::from_millis(100));
            serial.write_all("R,14,01\r\n".as_bytes()).unwrap();

        }
        ByteCommand::FiftyRonStacked => {
            serial.write_all("R,13,05C3500004\r\n".as_bytes()).unwrap();
            thread::sleep(Duration::from_millis(100));
            serial.write_all("R,14,01\r\n".as_bytes()).unwrap();
        }
    }
}

enum ByteCommand {
    OneRonStacked,
    FiveRonStacked,
    TenRonStacked,
    FiftyRonStacked,
}

static BYTE_TABLE: phf::Map<&'static str, ByteCommand>= phf_map! {
    "80" => ByteCommand::OneRonStacked,
    "81" => ByteCommand::FiveRonStacked,
    "82" => ByteCommand::TenRonStacked,
    "83" => ByteCommand::FiftyRonStacked
};