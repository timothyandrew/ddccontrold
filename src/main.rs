use std::str::FromStr;

use clap::{AppSettings, Clap};
use ddccontrold::{listen, set_brightness, set_contrast};

#[derive(Clap, Debug)]
enum Mode {
    Server,
    Client,
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "server" => Ok(Mode::Server),
            "client" => Ok(Mode::Client),
            _ => Err("no match"),
        }
    }
}

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Set the domain socket used to send commands through
    #[clap(short, long, default_value = "/tmp/.ddcontrol2.sock")]
    sock: String,
    /// Start up in server or client mode
    #[clap(short, long, default_value = "client")]
    mode: Mode,
    /// Set brightness [0-100]
    #[clap(short, long)]
    brightness: Option<usize>,
    /// Set contrast [0-100]
    #[clap(short, long)]
    contrast: Option<usize>,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.mode {
        Mode::Server => listen(&opts.sock),
        Mode::Client => {
            if let Some(value) = opts.brightness {
                set_brightness(&opts.sock, value)
            }

            if let Some(value) = opts.contrast {
                set_contrast(&opts.sock, value)
            }
        }
    }
}
