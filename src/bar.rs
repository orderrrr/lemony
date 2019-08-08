use super::util::Item;
use super::util::Globals;
use super::util::Inputs;

use std::thread;

pub struct Bar {
    items: (Vec<Item>, Vec<Item>, Vec<Item>,),
    // timer: u32, // Will be used soon
    globals: Globals,
    first_run: bool,
    animate: bool,
}

impl Bar {
    pub fn new(left: Vec<Inputs>, center: Vec<Inputs>, right: Vec<Inputs>, globals: Globals) -> Bar {
        let mut left_input: Vec<Item> = Vec::new();
        let one_color = globals.use_one_color;
        for input in left {
            left_input.push(Item::new(input, one_color, globals.fg, globals.bg));
        };
        let mut center_input: Vec<Item> = Vec::new();
        for input in center {
            center_input.push(Item::new(input, one_color, globals.fg, globals.bg));
        };
        let mut right_input: Vec<Item> = Vec::new();
        for input in right {
            right_input.push(Item::new(input, one_color, globals.fg, globals.bg));
        };
        Bar {
            items: (left_input, center_input, right_input),
            // timer: 0,
            globals: globals,
            first_run: false,
            animate: true,
        }
    }
    pub fn update(&mut self) {
        self.globals.update();
        // update items
        for item in &mut self.items.0 {
            item.update(&mut self.globals);
        }
        for item in &mut self.items.1 {
            item.update(&mut self.globals);
        }
        for item in &mut self.items.2 {
            item.update(&mut self.globals);
        }
        self.first_run = true;
        self.animate = true;
    }
    pub fn animate(&mut self) {
        for item in &mut self.items.0 {
            item.animate();
        }
        for item in &mut self.items.1 {
            item.animate();
        }
        for item in &mut self.items.2 {
            item.animate();
        }
    }
    pub fn to_bar(&mut self) -> String {
        // get input
        let globals = self.globals.to_bar();
        let color_args = self.globals.get_color_args();
        let mut left = (String::new(), 0);
        for item in &mut self.items.0 {
            // len will be the length of the output. to be translated to the bar
            let (output, len) = item.to_bar();
            left.0 += &output;
            left.1 += len;
        }
        let mut center = (String::new(), 0);
        for item in &mut self.items.1 {
            // len will be the length of the output. to be translated to the bar
            let (output, len) = item.to_bar();
            center.0 += &output;
            center.1 += len;
        }
        let mut right = (String::new(), 0);
        for item in &mut self.items.2 {
            // len will be the length of the output. to be translated to the bar
            let (output, len) = item.to_bar();
            right.0 += &output;
            right.1 += len;
        }
        // calculating the spacing
        if !((self.globals.desired_len as f32 - (left.1 + center.1+ right.1) as f32) < 0.0) {
            let padding_total = self.globals.desired_len - (left.1 + center.1 + right.1);
            let padding_left;
            let padding_right;
            if padding_total % 2 != 0 {
                padding_left = (padding_total / 2) + 1;
                padding_right = padding_total / 2;
            } else {
                padding_left = padding_total / 2;
                padding_right = padding_total / 2;
            }
            let padding_char = ' ';
            let mut string_left: String = String::new();
            let mut string_right: String = String::new();
            for _ in 0..padding_left {
                string_left.push(padding_char);
            }
            for _ in 0..padding_right {
                string_right.push(padding_char);
            }
            // adding everything together
            let output = globals.0 +
                "%{l}" +
                &left.0 +
                &color_args.0 +
                // &string_left +
                &color_args.1 +
                "%{c}" +
                &center.0 +
                &color_args.0 +
                // &string_right +
                &color_args.1 +
                "%{r}" +
                &right.0;
            thread::sleep(self.globals.sleep_time);
            output
        } else {
            panic!("total length of items is greater than desired length")
        }
    }
}
