use std::process::Command;
use std::time::{
    Instant,
    Duration,
};
use sysinfo::{SystemExt, System};

use dirs::home_dir;
use mpd::Client;
// use chrono::prelude::Local;
use chrono::{Utc};

use std::fs::File;
use std::io::{BufRead, BufReader};

use super::functions::*;
use super::anims::{
    pad_between,
    pad,
    carousel,
    bounce,
    stretch,
    summarize,
    battery_icon,
};

// structs
// ITEM \\
pub struct Item {
    input: Inputs,
    functions: Vec<Function>,
    icon: char,
    chars: Vec<char>,
    pub output: Vec<char>,
    pub args: Arg,
    len: u32,
    anim_index: u32,
    // padding (between icon and text && between text and border)
    pad: (bool, bool),
}
impl Item {
    pub fn new(input: Inputs, one_color: bool, fg: u32, bg: u32) -> Item {
        // defaults for Inputs
        let mut len = 13;
        let mut args = Arg::new(fg, bg, vec![]);
        let mut pad = (false, true);
        let mut anim_index = 1;
        let mut functions = vec![Function::Stretch, Function::Summarize];
        // changing values depending on type
        match input {
            Inputs::DwmWorkspace => {
                len = 12;
                functions = vec![];
                if !one_color {
                    args = Arg::new(7, 5, vec![]);
                }
            },
            Inputs::BspwmWorkspace => {
                len = 12;
                functions = vec![];
                if !one_color {
                    args = Arg::new(7, 5, vec![]);
                }
            },
            Inputs::DwmLayout => {
                len = 4;
                functions = vec![];
                if !one_color {
                    args = Arg::new(7, 5, vec![]);
                }
            },
            Inputs::Song => {
                len = 17;
                functions = vec![Function::Carousel, Function::Bounce];
                if !one_color {
                    args = Arg::new(7, 3, vec![]);
                }
                pad.0 = true;
            },
            Inputs::Window => {
                len = 18;
                functions = vec![Function::Carousel, Function::Bounce];
                if !one_color {
                    args = Arg::new(7, 8, vec![]);
                }
            },
            Inputs::Network => {
                len = 15;
                functions = vec![Function::Carousel, Function::Stretch];
                if !one_color {
                    args = Arg::new(7, 8, vec![]);
                }
                pad.0 = true;
            },
            Inputs::Battery => {
                len = 5;
                functions = vec![];
                if !one_color {
                    args = Arg::new(7, 2, vec![]);
                }
                anim_index = 0;
                pad.0 = true;
            },
            Inputs::Time => {
                len = 12;
                functions = vec![];
                if !one_color {
                    args = Arg::new(7, 1, vec![]);
                }
                anim_index = 0;
                pad.0 = true;
            },
            Inputs::Date => {
                len = 12;
                functions = vec![];
                if !one_color {
                    args = Arg::new(7, 1, vec![]);
                }
                anim_index = 0;
                pad.0 = true;
            },
            Inputs::Cpu => {
                len = 5;
                functions = vec![];
                if !one_color {
                    args = Arg::new(7, 2, vec![]);
                }
                anim_index = 0;
                pad.0 = true;
            },
            Inputs::CpuTemp => {
                len = 6;
                functions = vec![];
                if !one_color {
                    args = Arg::new(7, 2, vec![]);
                }
                pad.0 = true;
            }
            Inputs::Memory => {
                len = 5;
                functions = vec![];
                if !one_color {
                    args = Arg::new(7, 2, vec![]);
                }
                anim_index = 0;
                pad.0 = true;
            },
            Inputs::VolumePulse => {
                len = 5;
                functions = vec![];
                if !one_color {
                    args = Arg::new(7, 3, vec![]);
                }
                anim_index = 0;
                pad.0 = true;
            },
            Inputs::VolumeMpd => {
                len = 5;
                functions = vec![];
                if !one_color {
                    args = Arg::new(7, 3, vec![]);
                }
                anim_index = 0;
                pad.0 = true;
            },
            Inputs::ExpressVpn => {
                len = 15;
                functions = vec![];
                if !one_color {
                    args = Arg::new(7, 1, vec![]);
                }
                anim_index = 0;
                pad.0 = true;
            }
            Inputs::Brightness => {
                len = 5;
                functions = vec![];
                if !one_color {
                    args = Arg::new(7, 3, vec![]);
                }
                anim_index = 0;
                pad.0 = true;
            },
        };
        Item {
            input: input,
            functions: functions,
            icon: ' ',
            chars: Vec::new(),
            output: Vec::new(),
            args: args,
            len: len,
            pad: pad,
            anim_index: anim_index,
        }
    }
    pub fn update(&mut self, globals: &mut Globals) {
        let (chars, icon) = match self.input {
            Inputs::DwmWorkspace   => { get_workspace_dwm() },
            Inputs::DwmLayout      => { get_layout_dwm() },
            Inputs::BspwmWorkspace => { get_workspace_bspwm() },
            Inputs::Song           => { get_song(&mut globals.conn) },
            Inputs::VolumeMpd      => { get_volume_mpd(&mut globals.conn) },
            Inputs::Window         => { get_window() },
            Inputs::Network        => { get_network() },
            Inputs::Battery        => { get_battery() },
            Inputs::Time           => { get_time(&globals.date_time) },
            Inputs::Date           => { get_date(&globals.date_time) },
            Inputs::Cpu            => { get_cpu(&mut globals.sys) },
            Inputs::Memory         => { get_memory(&mut globals.sys) },
            Inputs::VolumePulse    => { get_volume_pulse() },
            Inputs::CpuTemp        => { get_cpu_temp(&mut globals.sys) },
            Inputs::ExpressVpn     => { get_expressvpn() },
            Inputs::Brightness     => { get_brightness() },
        };

        self.chars = chars;
        self.icon = icon;
        self.args.update(globals.colors.to_owned());
    }
    pub fn animate(&mut self) {
        self.output = Vec::new();
        match self.input {
            Inputs::Battery => {
                let (_, icon) = get_battery();
                if self.anim_index > 4 { self.anim_index = 0 }
                let percentage: String = self.chars.iter().collect();
                let mut percentage = percentage.trim().to_string();
                percentage.pop();
                let percentage = percentage.parse::<u32>().unwrap();
                self.icon = battery_icon(icon, percentage, self.anim_index);
            }
            _ => (),
        }
        // doing animations
        for function in &mut self.functions {
            match function {
                // adding + 3 to the length because carousel adds " | " to the end
                Function::Carousel if self.chars.len() as u32 >= self.len => {
                    if ((self.chars.len() + 3) as u32) < self.anim_index { self.anim_index = 1 }
                    self.output = carousel(self.anim_index, self.chars.clone(), self.len);
                    break;
                },
                Function::Carousel => {},
                Function::Bounce => {
                    let len = self.len - self.chars.len() as u32;
                    if self.anim_index > len * 2 { self.anim_index = 1 }
                    self.output = bounce(self.anim_index, self.chars.clone(), len);
                    break;
                },
                Function::Summarize => {
                    if self.chars.len() < self.len as usize + 1 {
                        self.output = summarize(self.chars.clone(), self.len)
                    }
                },
                Function::Stretch => {
                    // println!("chars len is {}, desired len is {}", self.chars.len(), self.len);
                    if self.chars.len() < self.len as usize + 1 {
                        self.output = stretch(self.chars.clone(), self.len)
                    }
                },
            }
        }
        if self.output == Vec::new() {
            self.output = self.chars.clone();
        }
        self.anim_index += 1;
    }
    pub fn to_bar(&mut self) -> (String, u32) {
        let mut data = self.args.args.0.clone();
        // icon and chars joined
        let mut chars_and_icon: Vec<char> = self.output.clone();
        if self.pad.0 {
            chars_and_icon = pad_between(chars_and_icon.clone());
        }
        if self.icon != ' ' {
            chars_and_icon.insert(0, self.icon);
        }
        if self.pad.1 {
            chars_and_icon = pad(chars_and_icon);
        }
        let len = chars_and_icon.len() as u32;
        data.append(&mut chars_and_icon);
        data.append(&mut self.args.args.1.clone());
        (data.iter().collect(), len)
    }
}
// GLOBALS \\
pub struct Globals {
    pub colors: Vec<String>,
    pub desired_len: u32,
    pub sleep_time: Duration,
    pub conn: mpd::Client,
    pub date_time: chrono::DateTime<chrono::Utc>,
    pub use_one_color: bool,
    pub instant: Instant,
    pub sys: System,
    pub fg: u32,
    pub bg: u32,
    colors_dir: String,
    args: Vec<String>,
}
impl Globals {
    pub fn new(len: u32, time: Duration, args: Vec<String>, one_color: bool, instant: Instant) -> Globals {
        // pywal colors
        // let colors = lines_from_file(home_dir().unwrap().to_string_lossy().to_string() + "/.cache/wal/colors");
        let colors_dir = home_dir().unwrap().to_string_lossy().to_string() + "/.cache/walmate/colors";
        let colors = lines_from_file(colors_dir.to_owned()); 
        // println!("colors are {:?}", colors);
        // runs mpd just incase it wasnt initialized
        command("mpd");
        let conn = Client::connect("127.0.0.1:6600").unwrap();
        Globals {
            colors: colors,
            desired_len: len,
            sleep_time: time,
            conn: conn,
            args: args,
            colors_dir: colors_dir,
            bg: 0,
            fg: 7,
            use_one_color: one_color,
            date_time: Utc::now(),
            // date_time: Local::now(),
            instant: instant,
            sys: System::new(),
        }
    }
    pub fn update(&mut self) {
        // pywal
        // self.colors = lines_from_file(home_dir().unwrap().to_string_lossy().to_string() + "/.cache/wal/colors");
        self.colors = lines_from_file(self.colors_dir.to_owned()); 
        // let secs: u64 = self.instant.elapsed().as_secs();
        // if secs > 100 {
            // self.instant = Instant::now();
        // }
        // if secs as f32 % 5.0 == 0.0 {
            self.date_time = Utc::now();
            self.sys.refresh_all();
        // }
    }
    pub fn to_bar(&mut self) -> (String, String) {
        // formatting other args
        let mut args = Vec::new();
        for arg in &mut self.args {
            args.push(("%{".to_string() + &arg + "}", "%{".to_string() + &arg + "-}"));
        };
        // setting the foreground and background
        let mut args_collected = (
            "%{F".to_string() + &self.colors[self.fg as usize] + "}" + "%{B" + &self.colors[self.bg as usize] + "}", 
            "%{F-}%{B-}".to_string());
        // applying other args supplied above
        for arg in args {
            args_collected.0 = args_collected.0 + &arg.0;
            args_collected.1 = args_collected.1 + &arg.1;
        }
        args_collected
    }
    pub fn get_color_args(&mut self) -> (String, String) {
        ("%{F".to_string() + &self.colors[self.fg as usize] + "}" + "%{B" + &self.colors[self.bg as usize] + "}", 
        "%{F-}%{B-}".to_string())
    }
}
pub struct Arg {
    fg: u32,
    bg: u32,
    other_args: Vec<String>,
    pub args: (Vec<char>, Vec<char>),
}

impl Arg {
    fn new(fg: u32, bg: u32, other_args: Vec<String>) -> Arg {
        Arg {
            fg: fg,
            bg: bg,
            other_args: other_args,
            args: (Vec::new(), Vec::new()),
        }
    }
    fn update(&mut self, colors: Vec<String>) {
        // formatting other args
        let mut args = Vec::new();
        for arg in &mut self.other_args {
            args.push(("%{".to_string() + &arg + "}", "%{".to_string() + &arg + "-}"));
        };
        // setting the foreground and background
        let mut args_collected = (
            "%{F".to_string() + &colors[self.fg as usize] + "}" + "%{B" + &colors[self.bg as usize] + "}", 
            "%{F-}%{B-}".to_string());
        // applying other args supplied above
        for arg in args {
            args_collected.0 = args_collected.0 + &arg.0;
            args_collected.1 = args_collected.1 + &arg.1;
        }
        let mut args_collected_char: (Vec<char>, Vec<char>) = (Vec::new(), Vec::new());
        args_collected_char.0: Vec<char> = args_collected.0.chars().collect();
        args_collected_char.1: Vec<char> = args_collected.1.chars().collect();
        self.args = args_collected_char;
    }
}

enum Function {
    // moves the text around the screen
    Carousel,
    // bounces the text back and forth
    Bounce,
    // if the text is too long adds "..." 
    // and removes characters
    Summarize,
    // lengthens the text to the desired length.
    // if specified
    Stretch,
}
// all possible inputs
#[derive(Debug)]
pub enum Inputs {
    DwmWorkspace,
    DwmLayout,
    BspwmWorkspace,
    Song,
    Window,
    Network,
    Battery,
    Time,
    Date,
    Cpu,
    CpuTemp,
    Memory,
    VolumePulse,
    VolumeMpd,
    ExpressVpn,
    Brightness,
}

// simple command command to clean up the code
pub fn command(command: &str) -> std::process::Output {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command given")
}

fn lines_from_file(filename: String) -> Vec<String> {
    let file = File::open(filename).expect("Unable to locate file given");
    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }
    lines
}
