pub fn get_input_code (byte_sequence: &str) -> &str {
    let input_code: &str = match byte_sequence.len().to_string().as_str() {
        "144" => &byte_sequence[84..86],
        "96" => &byte_sequence[36..38],
        _ => "ERROR"
    };

    return input_code;
}

pub fn get_input_type (byte_sequence: &String) -> &str {
    let input_type: &str = match byte_sequence.len().to_string().as_str() {
        "144" => &byte_sequence[88..90],
        "96" => "rpt",
        _ => "ERROR"
    };

    return input_type;
}

pub fn get_key(input_code: &str) -> &str {
    let key = match input_code {
        "01" => "[ESCAPE]",
        "02" => "1",
        "03" => "2",
        "04" => "3",
        "05" => "4",
        "06" => "5",
        "07" => "6",
        "08" => "7",
        "09" => "8",
        "0A" => "9",
        "0B" => "0",
        "0C" => "-",
        "0D" => "=",
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
        "1A" => "[",
        "1B" => "]",
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
        "27" => ";",
        "28" => "\'",
        "29" => "[GRAVE]",
        "2A" => "[LEFTSHIFT]",
        "2B" => "\\",
        "2C" => "z",
        "2D" => "x",
        "2E" => "c",
        "2F" => "v",
        "30" => "b",
        "31" => "n",
        "32" => "m",
        "33" => ",",
        "34" => ".",
        "35" => "/",
        "38" => "[LEFTALT]",
        "39" => " ",
        "3A" => "[CAPSLOCK]",
        _ => "[UNKNOWN]",
    };

    return key;
}
