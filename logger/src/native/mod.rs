use std::io::{stdout, Write};

pub(crate) fn print(s: &str) {
    let mut stdout = stdout().lock();
    stdout.write(s.as_bytes()).unwrap();
    stdout.flush().unwrap();
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
