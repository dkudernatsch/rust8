use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt;

pub struct Display {
    buffer: [u8; 256]
}

impl Debug for Display {
    
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{

        let mut string = String::new();
        let mut b_per_line = 8;
        for byte in &self.buffer[..]{
            let mut c = 0;
            //println!("{:b}", byte);
            
            while c < 8 {
                //println!("{:01b}", (byte << c) & 0b1000_0000 as u8);
                if (byte << c) & 0x80 as u8 == 0x80 as u8 {
                    string.push_str(&"#");
                }else {
                    string.push_str(&" ");
                }
                c += 1;
            }
            b_per_line -= 1;
            if b_per_line == 0 {
                string.push('\n');
                b_per_line = 8;
            }
        }
        write!(f, "{}", string)
    }
}

impl Display {
    pub fn new() -> Display{
        Display{buffer: [0xAA; 256]}
    }
}

#[derive(Debug)]
pub struct Keyboard{
    keys: u16
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard{keys:0}
    }

    pub fn key_pressed(&self, key: u8) -> bool {
        if key >= 16 {
            panic!("Requested not existing key: {}",key);
        }

        match self.keys >> key & 1 {
            1 => true,
            _ => false
        }
    }
}

