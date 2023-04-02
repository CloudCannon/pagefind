use std::{
    fmt::Debug,
    fs::{create_dir_all, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use console::{Style, Term};
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub enum LogLevel {
    Standard,
    Verbose,
}

#[derive(Debug, Clone)]
pub enum LogStyle {
    Info,
    Status,
    Warning,
    Error,
    Success,
}

#[derive(Clone)]
pub struct Logger {
    log_level: LogLevel,
    out: Option<Term>,
    err: Term,
    logfile: Option<PathBuf>,
}

macro_rules! plural {
    ($len:expr) => {
        match $len {
            1 => "",
            _ => "s",
        }
    };
}

impl Debug for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Logger")
            .field("log_level", &self.log_level)
            .finish()
    }
}

lazy_static! {
    static ref STATUS: Style = Style::new().cyan().bold();
    static ref WARN: Style = Style::new().yellow();
    static ref ERROR: Style = Style::new().red();
    static ref SUCCESS: Style = Style::new().green();
}

impl Logger {
    pub fn new(log_level: LogLevel, use_terminal: bool, logfile: Option<PathBuf>) -> Self {
        if let Some(filename) = &logfile {
            if let Some(parent) = filename.parent() {
                create_dir_all(parent).unwrap();
            }

            let mut file = File::create(filename).unwrap();
            file.write_all("Pagefind logging initialized\n".as_bytes())
                .expect("Logfile should be writable");
        }

        Self {
            log_level,
            out: if (use_terminal) {
                Some(Term::stdout())
            } else {
                None
            },
            err: Term::stderr(),
            logfile,
        }
    }

    pub fn info<S: AsRef<str>>(&self, msg: S) {
        self.log(msg, LogLevel::Standard, LogStyle::Info);
    }

    pub fn v_info<S: AsRef<str>>(&self, msg: S) {
        self.log(msg, LogLevel::Verbose, LogStyle::Info);
    }

    pub fn status<S: AsRef<str>>(&self, msg: S) {
        self.log(msg, LogLevel::Standard, LogStyle::Status);
    }

    pub fn v_status<S: AsRef<str>>(&self, msg: S) {
        self.log(msg, LogLevel::Verbose, LogStyle::Status);
    }

    pub fn warn<S: AsRef<str>>(&self, msg: S) {
        self.log(msg, LogLevel::Standard, LogStyle::Warning);
    }

    pub fn v_warn<S: AsRef<str>>(&self, msg: S) {
        self.log(msg, LogLevel::Verbose, LogStyle::Warning);
    }

    pub fn error<S: AsRef<str>>(&self, msg: S) {
        self.log(msg, LogLevel::Standard, LogStyle::Error);
    }

    pub fn success<S: AsRef<str>>(&self, msg: S) {
        self.log(msg, LogLevel::Standard, LogStyle::Success);
    }

    pub fn log<S: AsRef<str>>(&self, msg: S, log_level: LogLevel, log_style: LogStyle) {
        let log = match log_level {
            LogLevel::Standard => true,
            LogLevel::Verbose => matches!(self.log_level, LogLevel::Verbose),
        };

        if let Some(filename) = &self.logfile {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(filename)
                .unwrap();

            writeln!(file, "[{log_style:?}]: {}", msg.as_ref()).unwrap();
        }

        if log {
            if let Some(out) = &self.out {
                // We currently aren't worried about logging failures.
                match log_style {
                    LogStyle::Info => {
                        let _ = out.write_line(msg.as_ref());
                    }
                    LogStyle::Status => {
                        let _ = out.write_line(&format!("\n{}", STATUS.apply_to(msg.as_ref())));
                    }
                    LogStyle::Warning => {
                        let _ = self
                            .err
                            .write_line(&WARN.apply_to(msg.as_ref()).to_string());
                    }
                    LogStyle::Error => {
                        let _ = self
                            .err
                            .write_line(&ERROR.apply_to(msg.as_ref()).to_string());
                    }
                    LogStyle::Success => {
                        let _ = out.write_line(&SUCCESS.apply_to(msg.as_ref()).to_string());
                    }
                };
            }
        }
    }
}
