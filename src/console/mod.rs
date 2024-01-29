use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::LOCAL_DEBUG;

pub fn print_info(message: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
    write!(&mut stdout, "[INFO] ").unwrap();
    let _ = stdout.reset();
    writeln!(&mut stdout, "{}", message).unwrap();  // Removed format_args!
}
#[allow(dead_code)]
pub fn print_warn(message: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
    write!(&mut stdout, "[WARN] ").unwrap();  // Changed "[INFO]" to "[WARN]"
    let _ = stdout.reset();
    writeln!(&mut stdout, "{}", message).unwrap();  // Removed format_args!
}

pub fn print_error(message: &str) -> &'static str {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
    write!(&mut stdout, "[ERROR] ").unwrap();
    let _ = stdout.reset();
    writeln!(&mut stdout, "{}", message).unwrap();
    let err = format!("[ERROR] {}", message);
    Box::leak(err.into_boxed_str())
}

pub fn print_info_debug(message: &str) {
    if LOCAL_DEBUG{
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)));
        write!(&mut stdout, "[DEBUG] ").unwrap();
        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
        write!(&mut stdout, "[INFO] ").unwrap();
        let _ = stdout.reset();
        writeln!(&mut stdout, "{}", message).unwrap();
    }
}