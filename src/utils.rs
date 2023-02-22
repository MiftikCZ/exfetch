use std::string;

pub fn point_to(x:i32,y:i32) {
    print!("{esc}[2J{esc}[{y};{x}H", esc = 27 as char);
}

pub fn clear() {
    //print!("{esc}c", esc = 27 as char);
    std::process::Command::new("clear").status().unwrap();
}

pub fn align(width:usize,word:&str,_align:&str) {
    match _align {
        "start"  => print!("{:<width$}", word, width=width),
        "center" => print!("{:^width$}", word, width=width),
        _        => print!("{:>width$}", word, width=width)
    }
}