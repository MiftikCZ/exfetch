mod utils;
extern crate termsize;
use std::{any, string,io,env};
use nixinfo::{
    cpu, device, distro, env, environment, gpu, hostname, kernel, memory, music, packages,
    terminal, uptime,
};
use termion::{color::{self, Fg}, style};

fn item(title:&str, out:&str,width:usize) {
    print!("{}",color::Fg(color::Green));
    utils::align(width, title, "center");
    print!("{}{}",color::Fg(color::Reset),out);
    println!();
}
//
fn main() {
    let def_w_1 = 12;
    let args: Vec<String> = env::args().collect();

    print!("{}",color::Fg(color::LightBlack));
    utils::align(def_w_1, "[...]", "center");

    print!("{}",color::Fg(color::AnsiValue(12)));
    println!("Hello {} !",
    env("USER").unwrap().as_str());


    item("", distro().unwrap().as_str(), def_w_1);
    item("", kernel().unwrap().as_str(), def_w_1);
    item("", terminal().unwrap().as_str(), def_w_1);
    item("", environment().unwrap().as_str(), def_w_1);
    item("", packages("pacman").unwrap().as_str(), def_w_1);
    item("", uptime().unwrap().as_str(), def_w_1);
    item("", memory().unwrap().as_str(), def_w_1);
    print!("\n");
    for n in 0..16 {
        print!("{}  {}",color::Fg(color::AnsiValue(n)),color::Fg(color::Reset));
        if (n == 7) {
            println!("");
        }
    }
    println!("{}{}",
        color::Fg(color::Reset),
        color::Bg(color::Reset));
}

