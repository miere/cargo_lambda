use termion::{color, style};

pub fn error(msg: String) {
    println!("{}{}error:{} {}{}", color::Fg(color::Red), style::Bold, color::Fg(color::White), msg, style::Reset)
}

pub fn warn(msg: String) {
    println!("{}{}warning:{} {}{}", color::Fg(color::Yellow), style::Bold, color::Fg(color::White), msg, style::Reset)
}

pub fn info(msg: String) {
    println!("{}{}{}", style::Bold, msg, style::Reset)
}