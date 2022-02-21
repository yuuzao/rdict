use colored::Colorize;

pub fn colorize(s: &str, rgb: (u8, u8, u8)) -> String {
    s.truecolor(rgb.0, rgb.1, rgb.2).to_string()
}
