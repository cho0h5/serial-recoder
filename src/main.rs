use std::time::Duration;
use std::io::BufReader;
use std::io::BufRead;
use chrono::Local;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let port = serialport::new("/dev/ttyACM0", 9600)
        .timeout(Duration::from_millis(20000))
        .open().expect("Failed to open port");

    let mut reader = BufReader::new(port);
    let mut data = String::new();

    loop {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("data.csv")
            .unwrap();

        data.clear();
        reader.read_line(&mut data)?;
        println!("{}, {}", Local::now().format("%Y-%m-%d %H:%M:%S"), data.trim());
        writeln!(file, "{}, {}", Local::now().format("%Y-%m-%d %H:%M:%S"), data.trim());
    }
}
