#![allow(dead_code)]

use {
    std::{
        fmt::Display,
        io::Write,
    },
    termcolor::{
        Color,
        ColorChoice,
        ColorSpec,
        StandardStream,
        WriteColor,
    },
};

#[macro_export]
macro_rules! print_status {
    ($action:expr, $($args: tt)*) => {
        print_style($action, &format!($($args)*), termcolor::Color::Green, true)
    };
}

#[macro_export]
macro_rules! print_note {
     ($($args: tt)*) => {
        print_style("note", &format!($($args)*), termcolor::Color::Cyan, true)
    };
}

#[macro_export]
macro_rules! print_warn {
     ($($args: tt)*) => {
        print_style("warning", &format!($($args)*), termcolor::Color::Yellow, false)
    };
}

#[macro_export]
macro_rules! print_error {
     ($($args: tt)*) => {
        print_style("ERROR", &format!($($args)*), termcolor::Color::Red, false)
    };
}

/// Print a message with a colored title in the style of Cargo shell messages.
pub fn print_style<S: AsRef<str> + Display>(status: S, message: S, color: Color, justified: bool) {
    let mut output = StandardStream::stderr(ColorChoice::Auto);
    output
        .set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))
        .unwrap();
    if justified {
        write!(output, "{status:>12}").unwrap();
    } else {
        write!(output, "{status}").unwrap();
        output.set_color(ColorSpec::new().set_bold(true)).unwrap();
        write!(output, ":").unwrap();
    }
    output.reset().unwrap();
    writeln!(output, " {message}").unwrap();
}
