#![feature(uniform_paths)]
#![feature(type_ascription)]

extern crate dirs;
extern crate mpd;
extern crate chrono;
extern crate sysinfo;

use std::time::{
    Duration,
    Instant,
};

mod util;
mod functions;
mod bar;
mod anims;

use util::{Inputs::*, Globals};
use bar::{Bar};

fn main() {
    let instant = Instant::now();
    let left   = vec![DwmWorkspace, DwmLayout, Song, VolumeMpd];
    let center = vec![Date, Time, Window];
    let right  = vec![Network, ExpressVpn, VolumePulse, Brightness, CpuTemp, Cpu, Memory, Battery];
    // let right  = vec![Network, ExpressVpn, VolumePulse, CpuTemp, Cpu, Memory, Battery];
    // let right  = vec![Network, ExpressVpn, VolumePulse]; //CpuTemp, Cpu, Memory, Battery];
    let globals = Globals::new(208, Duration::from_millis(300), vec!["c".to_string(), "S0".to_string()], false, instant);
    let mut bar = Bar::new(left, center, right, globals);
    loop {
        bar.update();
        bar.animate();
        println!("{}", bar.to_bar());
    }
}

// 
