use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{stdout, BufWriter, Write},
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

use chrono::Local;

use crate::LoggerCallback;

static LOG_FILE: LazyLock<Mutex<BufWriter<File>>> = LazyLock::new(|| {
    let timestamp = Local::now().format("%Y.%m.%d at %H.%M.%S.txt").to_string();
    let log_dir = PathBuf::from("logs");

    create_dir_all(&log_dir).expect("Failed to create logs directory");

    let filepath = log_dir.join(timestamp);

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filepath)
        .expect("Failed to open log file");

    Mutex::new(BufWriter::new(file))
});

pub struct DefaultLoggerCallback;

impl LoggerCallback for DefaultLoggerCallback {
    fn print_to_console(&self, s: &str) {
        let mut stdout = stdout().lock();
        stdout.write(s.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }

    fn print_to_logfile(&self, s: &str) {
        let mut file = LOG_FILE.lock().unwrap();
        file.write_all(s.as_bytes()).unwrap();
        file.flush().unwrap();
    }
}

pub(crate) fn time_string() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub(crate) macro ansi_code {
    (reset) => {
        0
    },
    (bold) => {
        1
    },

    (fg_black) => {
        30
    },
    (fg_red) => {
        31
    },
    (fg_green) => {
        32
    },
    (fg_yellow) => {
        33
    },
    (fg_blue) => {
        34
    },
    (fg_magenta) => {
        35
    },
    (fg_cyan) => {
        36
    },
    (fg_white) => {
        37
    },
    (fg_default) => {
        39
    },

    (bg_black) => {
        40
    },
    (bg_red) => {
        41
    },
    (bg_green) => {
        42
    },
    (bg_yellow) => {
        43
    },
    (bg_blue) => {
        44
    },
    (bg_magenta) => {
        45
    },
    (bg_cyan) => {
        46
    },
    (bg_white) => {
        47
    },
    (bg_default) => {
        49
    },

    ($n: literal) => {
        $n
    },
}

pub(crate) macro ansi ($($first: tt $(, $rest: tt)* $(,)?)?) {
    concat!($("\x1b[", ansi_code!($first) $(, ";", ansi_code!($rest))*, "m")?)
}
