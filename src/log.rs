macro_rules! log {
    () => {{
        if std::env::var("APT_LISTBUG_RS_VERBOSE").is_ok() {
            println!("[apt-listbugs-rs] \n")
        }
    }};
    ($($arg:tt)*) => {{
        if std::env::var("APT_LISTBUG_RS_VERBOSE").is_ok() {
            println!("[apt-listbugs-rs] {}", std::format_args!($($arg)*));
        }
    }};
}
pub(crate) use log;
