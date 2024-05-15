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

pub fn get_key_shift(input_code: &str) -> &str {
    let key = match input_code {
        "01" => "[SHIFT+ESCAPE]",
        "02" => "!",
        "03" => "@",
        "04" => "#",
        "05" => "$",
        "06" => "%",
        "07" => "^",
        "08" => "&",
        "09" => "*",
        "0A" => "(",
        "0B" => ")",
        "0C" => "_",
        "0D" => "+",
        "0E" => "[SHIFT+BACKSPACE]",
        "0F" => "[SHIFT+TAB]",
        "10" => "Q",
        "11" => "W",
        "12" => "E",
        "13" => "R",
        "14" => "T",
        "15" => "Y",
        "16" => "U",
        "17" => "I",
        "18" => "O",
        "19" => "P",
        "1A" => "{",
        "1B" => "}",
        "1C" => "[SHIFT+ENTER]",
        "1D" => "",
        "1E" => "A",
        "1F" => "S",
        "20" => "D",
        "21" => "F",
        "22" => "G",
        "23" => "H",
        "24" => "J",
        "25" => "K",
        "26" => "L",
        "27" => ":",
        "28" => "\"",
        "29" => "[SHIFT+GRAVE]",
        "2A" => "",
        "2B" => "|",
        "2C" => "Z",
        "2D" => "X",
        "2E" => "C",
        "2F" => "V",
        "30" => "B",
        "31" => "N",
        "32" => "M",
        "33" => "<",
        "34" => ">",
        "35" => "?",
        "38" => "[SHIFT+LEFTALT]",
        "39" => " ",
        "3A" => "[SHIFT+CAPSLOCK]",
        _ => "[UNKNOWN]",
    };

    return key;
}
