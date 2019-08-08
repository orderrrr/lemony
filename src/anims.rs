pub fn carousel(i: u32, chars: Vec<char>, len: u32) -> Vec<char> {
    let mut chars = chars.clone();
    chars.push(' ');
    chars.push('|');
    chars.push(' ');
    if (i + len) >= (chars.len()) as u32 {
        let mut new_chars = chars[i as usize..].to_vec();
        let remainder = len - new_chars.len() as u32;
        let end_chars = &chars[0..remainder as usize].to_vec().to_owned();
        new_chars.append(&mut end_chars.to_owned());
        // let new_string: String = new_chars.iter().collect();
        // new_string
        new_chars
    } else {
        let new_chars = chars[i as usize..(i + len) as usize].to_vec();
        // let new_string: String = new_chars.iter().collect();
        // new_string
        new_chars
    }
}
pub fn bounce(i: u32, chars: Vec<char>, len: u32) -> Vec<char> {
    let mut output = chars;
    if i >= len {
        let i = i - len;
        let remainder = len - i;
        for _ in 0..i { output.push(' '); }
        for _ in 0..remainder { output.insert(0, ' '); }
    } else {

        let remainder = len - i;
        for _ in 0..i { output.insert(0, ' '); }
        for _ in 0..remainder { output.push(' '); }
    }
    output
}
pub fn summarize(characters: Vec<char>, len: u32) -> Vec<char> {
    // assumes the given char is shorter than the length
    let mut shortened = characters[..(len - 3) as usize].to_vec();
    let dots: Vec<char> = ("...".to_string()).chars().collect();
    shortened.extend(dots.iter());
    shortened
}
pub fn stretch(characters:Vec<char>, len: u32) -> Vec<char> {
    // println!("len is {}. characters are {:?}", len, characters);
    let mut characters = characters.clone();
    let diff = len - characters.len() as u32;
    for _ in 0..diff {
        characters.push(' ');
    }
    characters
}
pub fn pad_between(mut chars: Vec<char>) -> Vec<char> {
    chars.insert(0, ' ');
    chars
}
pub fn pad(mut chars: Vec<char>) -> Vec<char> {
    chars.insert(0, ' ');
    chars.push(' ');
    chars
}
pub fn battery_icon(icon: char, percentage: u32, i: u32) -> char {
    let icon = match icon {
        'C' => {
            match i {
                0 => '',
                1 => '',
                2 => '',
                3 => '',
                4 => '',
                _ => 'C',
            }
        },
        'D' => {
            match percentage {
                x if x <= 20 => '',
                x if x > 20 && x <= 40 => '',
                x if x > 40 && x <= 60 => '',
                x if x > 60 && x <= 80 => '',
                x if x > 80 => '',
                _ => 'D',
            }
        },
        'F' => '',
        _ => 'F',
    };
    icon
}
