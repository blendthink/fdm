#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use colored::Colorize;
        let colored = format!($($arg)*).red();
        println!("{}", colored)
    }};
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{
        use colored::Colorize;
        let colored = format!($($arg)*).yellow();
        println!("{}", colored)
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use colored::Colorize;
        let colored = format!($($arg)*).magenta();
        println!("{}", colored)
    }};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        use colored::Colorize;
        let colored = format!($($arg)*).green();
        eprintln!("[{}:{}] {}", file!(), line!(), colored)
    }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! progress {
    ($len:expr) => {{
        console::app_progress($len)
    }};
}

pub use debug;
pub use error;
pub use info;
pub use progress;
pub use warning;
