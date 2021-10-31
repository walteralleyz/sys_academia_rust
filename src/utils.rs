use std::io::stdin;

pub fn input_string(message: String) -> String {
    let mut temp = String::new();

    println!("{}", message);

    return match stdin().read_line(&mut temp) {
        Ok(_) => temp.trim().to_string(),
        Err(_e) => "".to_string()
    };
}

pub fn input_u8(message: String) -> u8 {
    let to_parse = input_string(message);
    to_parse.parse().unwrap()
}

pub fn input_f32(message: String) -> f32 {
    let to_parse = input_string(message);
    to_parse.parse().unwrap()
}