use super::util::command;

use chrono::{Timelike, Datelike};
use sysinfo::{ProcessorExt, SystemExt, System, ComponentExt};

pub fn get_workspace_dwm() -> (Vec<char>, char) {
    let workspace = match (String::from_utf8_lossy(&command("cat /tmp/dwm_info/current_ws").stdout).to_string()).trim().to_string().as_ref() {
        "1" => (String::from(": Terminals"), ''),
        "2" => (String::from(": Browsers "), ''),
        "3" => (String::from(": Editors  "), ''),
        "4" => (String::from(": Explorers"), ''),
        "5" => (String::from(": Gaming   "), ''),
        "6" => (String::from(": Misc     "), ''),
        "7" => (String::from(": Sound    "), ''),
        "8" => (String::from(": Empty    "), ''),
        "9" => (String::from(": Empty2   "), ''),
        _   => (String::from(": Unknown  "), ''),
    };
    (workspace.0.chars().collect(), workspace.1)
}
pub fn get_layout_dwm() -> (Vec<char>, char) {
    let current_layout = match (String::from_utf8_lossy(&command("cat /tmp/dwm_info/current_layout").stdout).to_string()).trim().to_string().as_ref() {
        "0" => String::from("[]="),
        "1" => String::from("><>"),
        "2" => String::from("[M]"),
        "3" => String::from("|M|"),
        "4" => String::from("|m|"),
        "5" => String::from("==="),
        "6" => String::from("[f]"),
        _   => String::from("|?|")
    };
    (current_layout.chars().collect(), '-')
}
pub fn get_workspace_bspwm() -> (Vec<char>, char) {
    let workspace = match (String::from_utf8_lossy(&command("bspc query -D -d focused --names").stdout)).to_string().trim() {
        "I"     => (String::from(": Terminals"), ''),
        "II"    => (String::from(": Browsers "), ''),
        "III"   => (String::from(": Editors  "), ''),
        "IV"    => (String::from(": Explorers"), ''),
        "V"     => (String::from(": Gaming   "), ''),
        "VI"    => (String::from(": Misc     "), ''),
        "VII"   => (String::from(": Empty    "), ''),
        "VIII"  => (String::from(": Sound    "), ''),
        "IX"    => (String::from(": #2Monitor"), ''),
        _       => (String::from(": Unknown  "), ''),
    };
    (workspace.0.chars().collect(), workspace.1)
}
pub fn get_workspace_openbox() -> (Vec<char>, char) {
    let workspace = match (String::from_utf8_lossy(&command("xprop -root _NET_CURRENT_DESKTOP | awk '{print $3}'").stdout).to_string()).trim().to_string().as_ref() {
        "0" => (String::from("|    "), ''),
        "1" => (String::from("|    "), ''),
        "2" => (String::from("|    "), ''),
        "3" => (String::from("|    "), ''),
        _   => (String::from(": Unknown  "), ''),
    };
    (workspace.0.chars().collect(), workspace.1)
}

pub fn get_song(conn: &mut mpd::Client) -> (Vec<char>, char) {
    let status = conn.status().unwrap().state;
    if (status == mpd::State::Stop) || (status == mpd::State::Pause) {
        (String::from("Paused").chars().collect(), '')
    } else {
        let string = match conn.currentsong().unwrap().unwrap().title {
            Some(e) => {
                e + " - " + &{
                    if conn.currentsong().unwrap().unwrap().tags.contains_key("Artist") {
                        conn.currentsong().unwrap().unwrap().tags["Artist"].to_owned()
                    } else {
                        "unknown".to_string()
                    }
                }
            },
            _ => {
                let string = conn.currentsong().unwrap().unwrap().file.to_owned();
                let string: Vec<&str> = string.split("/").collect();
                let string = string[string.len() - 1];
                string.to_string()
            },
        };
        (string.chars().collect(), '')
    }
}
pub fn get_song_mpc() -> (Vec<char>, char) {
    let song = (String::from_utf8_lossy(&command("mpc --format '[%title% > %artist%]' current").stdout)).to_string().trim().chars().collect();
    (song, '')
}

pub fn get_window() -> (Vec<char>, char) {
    let window = (String::from_utf8_lossy(&command("xdotool getwindowfocus getwindowname").stdout)).to_string().trim().chars().collect();
    (window, ' ')
}
pub fn get_network() -> (Vec<char>, char) {
    let enp_or_wifi = match (String::from_utf8_lossy(&command("cat /sys/class/net/enp58s0u1u4/carrier").stdout).to_string()).trim().to_string().parse::<u32>() {
        Ok(x) => x,
        Err(_) => {
            match (String::from_utf8_lossy(&command("cat /sys/class/net/enp61s0/carrier").stdout).to_string()).trim().to_string().parse::<u32>() {
                Ok(x) => x,
                Err(_) => 0,
            }
        }
    };
    match enp_or_wifi {
        1 => {
            let ip: String = String::from_utf8_lossy(&command("ip a | grep 'inet ' | head -2").stdout).trim().to_owned();
            let ip: Vec<&str> = ip.split("inet ").collect();
            let ip = ip[2];
            let ip: Vec<&str> = ip.split("/").collect();
            let ip = (ip[0]).trim().trim();
            let ip = ip.chars().collect();
            (ip, '')
        },
        0 => {
            // get ssid and set icon to wifi
            let ssid: String = String::from_utf8_lossy(&command("iwgetid").stdout).trim().to_string();
            let ssid: Vec<char> = match ssid.len() {
                x if x > 0 => {
                    let ssid: Vec<&str> = ssid.split('"').collect();
                    let ssid = String::from(ssid[1]);
                    ssid.chars().collect()
                },
                _ => "Not Connected".to_string().chars().collect(),
            };
            (ssid, '')
        },
        _ => {
            let characters = "unknown".chars().collect();
            (characters, '?')
        }
    }
}
pub fn get_battery() -> (Vec<char>, char) {
    let battery: String = (String::from_utf8_lossy(&command("acpi -b | tail -1").stdout).to_string()).trim().to_string();
    let separated: Vec<&str> = battery.split(',').collect();
    let mut percentage = separated[1].to_string().trim().to_string();
    percentage.pop();
    let mut percentage = percentage.parse::<u32>().unwrap();
    if percentage >= 100 {
        percentage = 99;
    }
    let separated: Vec<&str> = separated[0].split(":").collect();
    let state = separated[1].trim().to_string();
    let state = match state.trim() {
        "Full" => 'F',
        "Discharging" => 'D',
        "Charging" => 'C',
        _ => 'U',
    };
    // icon moved to anims
    // let percentage = get_battery_chars(percentage);
    let percentage: Vec<char> = (percentage.to_string() + "%").chars().collect();
    (percentage, state)
}

pub fn get_time(dt: &chrono::DateTime<chrono::Utc>) -> (Vec<char>, char) {
    let hour = dt.hour();
    let minute = dt.minute();
    let second = dt.second();
    let mut hour_string: String;
    let mut minute_string: String;
    let mut second_string: String;
    if hour < 10 {
        hour_string = "0".to_string() + &hour.to_string();
    } else {
        hour_string = hour.to_string();
    }
    if minute < 10 {
        minute_string = "0".to_string() + &minute.to_string();
    } else {
        minute_string = minute.to_string();
    }
    if second < 10 {
        second_string = "0".to_string() + &second.to_string();
    } else {
        second_string = second.to_string();
    }
    let time = format!("{}:{}:{}", hour_string, minute_string, second_string).chars().collect();

    // setting hour to 24 for the clock icons
    let mut hour = dt.hour();
    if hour > 12 {
        hour = hour - 12;
    }
    let icon = match hour {
        1  => '', 2  => '', 3  => '',
        4  => '', 5  => '', 6  => '',
        7  => '', 8  => '', 9  => '',
        10 => '', 11 => '', 12 => '',
        _  => '',
    };
    (time, icon)
}
pub fn get_date(dt: &chrono::DateTime<chrono::Utc>) -> (Vec<char>, char) {
    // adding a zero just in case the day is a single digit
    let mut chars;
    if (dt.day().to_string().chars().collect(): Vec<char>).len() < 2 {
        chars = dt.format("%a %b %d").to_string().chars().collect();
    } else {
        chars = dt.format("%a %b %e").to_string().chars().collect();
    }

    let c = 365.25 * dt.year() as f32;
    let e = 30.6 * dt.month() as f32;
    let mut jd = c + e + dt.day() as f32 - 694039.09; //jd is total days elapsed
    jd /= 29.5305882; //divide by the moon cycle
    let mut b = jd as u64; //int(jd) -> b, take integer part of jd
    jd -= b as f32; //subtract integer part to leave fractional part of original jd
    b = ((jd * 29.0).round()) as u64; //scale fraction from 0-8 and round
    if b >= 29  {
        b = 0; //0 and 8 are the same so turn 8 into 0
    }
    // 
    let icon = match b {
        0  => '', 1  => '', 2  => '',
        3  => '', 4  => '', 5  => '',
        6  => '', 7  => '', 8  => '',
        9  => '', 10 => '', 11 => '',
        12 => '', 13 => '', 14 => '',
        15 => '', 16 => '', 17 => '',
        18 => '', 19 => '', 20 => '',
        21 => '', 22 => '', 23 => '',
        24 => '', 25 => '', 26 => '',
        27 => '', _ => '?',
    };
    (chars, icon) //''
}
pub fn get_cpu(sys: &mut System) -> (Vec<char>, char) {
    let mut cpu = 0.0;
    for (i, processor) in sys.get_processor_list().iter().enumerate() {
        if i == 0 {
            cpu = processor.get_cpu_usage();
        }
    }
    let mut percentage = (cpu * 100.0) as u32;
    if percentage >= 100 {
        percentage = 99;
    }
    let chars = if percentage < 10 {
        ("0".to_string() + &percentage.to_string() + "%").chars().collect()
    } else {
        (percentage.to_string() + "%").chars().collect()
    };
    (chars, '')
}
pub fn get_memory(sys: &mut System) -> (Vec<char>, char) {
    let total_mem = sys.get_total_memory();
    let used_mem = sys.get_used_memory();
    let mut percentage = ((used_mem as f32 / total_mem as f32) * 100.0) as u32;
    if percentage >= 100 {
        percentage = 99;
    }
    let chars = if percentage < 10 {
        ("0".to_string() + &percentage.to_string() + "%").chars().collect()
    } else {
        (percentage.to_string() + "%").chars().collect()
    };
    (chars, '')
}
pub fn get_volume_pulse() -> (Vec<char>, char) {
    let volume = String::from_utf8_lossy(&command(r#"pactl list sinks | grep '^[[:space:]]Volume:' | \head -n $(( $SINK + 1 )) | tail -n 1 | sed -e 's,.* \([0-9][0-9]*\)%.*,\1,'"#).stdout).trim().parse::<u32>().unwrap();
    // check if muted
    let muted = String::from_utf8_lossy(&command("pactl list sinks | grep '^[[:space:]]Mute:' | head -n $(( $SINK + 1 )) | tail -n 1 | sed -e 's,.* ([0-9][0-9]*)%.*,1,'").stdout).trim().to_string();
    let collected: Vec<&str> = muted.split(":").collect();
    let muted = match collected[1].trim() {
        "yes" => true,
        "no" => false,
        _ => false,
    };

    let icon = match muted {
        true => '',
        false => {
            match volume {
                x if x <= 33 => '',
                x if x > 33 && x <= 66 => '',
                x if x > 66 => '',
                _ => ''
            }
        },
    };
    let chars = if volume < 10 {
        ("0".to_string() + &volume.to_string()).chars().collect()
    } else if volume >= 100 {
        let volume = 99;
        volume.to_string().chars().collect()
    } else {
        volume.to_string().chars().collect()
    };
    (chars, icon)
}
pub fn get_volume_mpd(conn: &mut mpd::Client) -> (Vec<char>, char) {
    let volume = conn.status().unwrap().volume;
    let icon = match volume {
        x if x <= 33 => '',
        x if x > 33 && x <= 66 => '',
        x if x > 66 => '',
        _ => ''
    };
    let chars = if volume < 10 {
        ("0".to_string() + &volume.to_string()).chars().collect()
    } else if volume >= 100 {
        let volume = 99;
        volume.to_string().chars().collect()
    } else {
        volume.to_string().chars().collect()
    };
    (chars, icon)
}

pub fn get_cpu_temp(sys: &mut System) -> (Vec<char>, char) {
    let mut comp = 0;
    for (i, component) in sys.get_components_list().iter().enumerate() {
        if i == 4 {
            comp = component.get_temperature() as u32;
            // println!("{:?}", component);
        }
    }
    let icon = match comp {
        x if x <= 60 => '',
        x if x > 60 && x <= 70 => '',
        x if x > 70 && x <= 80 => '',
        x if x > 80 && x <= 90 => '',
        x if x > 90 => '',
        _ => '?',
    };
    let chars = match comp {
        x if x < 10 => "0".to_string() + &x.to_string() + "糖",
        mut x if x >= 100 => {
            x = 99;
            x.to_string() + "糖"
        },
        x => x.to_string() + "糖",
    }.chars().collect();
    (chars, icon)
}
pub fn get_expressvpn() -> (Vec<char>, char) {
    let status = String::from_utf8_lossy(&command("expressvpn status | head -n 1").stdout).trim().to_string();
    let status = status.replace('\n', "");
    let chars: Vec<char>;
    let icon; // 
    if status.contains("Not connected") {
        chars = "Not connected".to_string().chars().collect();
        icon = '';
    } else if status.contains("Connecting") {
        chars = "Connecting...".chars().collect();
        icon = '';
    } else if status.contains("Connected") {
        chars = (status.chars().collect(): Vec<char>)[23..].to_vec();
        // chars = (status.chars().collect(): Vec<char>)[23..33].to_vec();
        icon = '';
    } else if status.contains("Reconnecting") {
        chars = "Reconnecting.".chars().collect();
        icon = '';
    } else {
        chars = "Error...     ".chars().collect();
        icon = '?';
    }
    (chars, icon)
}

pub fn get_brightness() -> (Vec<char>, char) {
    let brightness: String = (String::from_utf8_lossy(&command("xbacklight").stdout)).to_string().trim().to_string();
    let brightness = match brightness.parse::<f32>() {
        Ok(x) => x as u32,
        Err(_) => 00,
    };

    let brightness = if brightness < 10 {
        "0".to_string() + &brightness.to_string()
    } else if brightness > 99 {
        99.to_string()
    } else {
        brightness.to_string()
    }.chars().collect();
    // 
    (brightness, '')
}
