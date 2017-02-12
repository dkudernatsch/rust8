use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt;

extern crate bit_vec;
use bit_vec::BitVec;

pub struct Display {
    buffer: BitVec
}

impl Debug for Display {
    
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{

        let mut string = String::new();
        let mut b_per_line = 64;
        string.push_str("+----------------------------------------------------------------+\n|");
        for bit in self.buffer.iter(){
            if bit {
                string.push('#');
            }else {
                string.push(' ');
            }
            b_per_line -= 1;
            if b_per_line == 0 {
                b_per_line = 64;
                string.push_str("|\n|");
            }
        }
        string.pop();
        string.pop();
        string.push_str("\n+----------------------------------------------------------------+");
        write!(f, "{}", string)
    }
}

impl Display {
    pub fn new() -> Display{
        Display{buffer: BitVec::from_bytes(&[0 as u8; 256])}
    }

    pub fn clear(&mut self) {
        self.buffer.set_all();
        self.buffer.negate();
    }

   pub fn draw (&mut self, sprite: &[u8], pos_x: u8, pos_y: u8) -> bool {
        println!("drawing: {:?}", sprite);
        let pos: usize = pos_x as usize + 64 * pos_y as usize;
        let mut ret = false;       
        for (line, sprite_line) in sprite.iter().enumerate() {
            let mut bit_in_byte: u8 = 8;
            while bit_in_byte > 0 as u8 {
                println!("{:?}", bit_in_byte);
                bit_in_byte -= 1;
                ret |= match sprite_line >> bit_in_byte & 0x01 {
                    1 => self.set_pixel(true,  pos_x + 7 - bit_in_byte, pos_y + line as u8),
                    0 => self.set_pixel(false, pos_x + 7 - bit_in_byte, pos_y + line as u8),
                    _ => false
                }
            }
       }
       ret
   }

   fn set_pixel(&mut self, p: bool, x: u8, y: u8) -> bool {
       let pos = y as usize * 64 + x as usize;
       let p_e = self.buffer.get(pos);

       match p_e {
           Some(p_e) => {
               self.buffer.set(pos, p_e ^ p);
                p_e == true && p_e ^ p == false 
               },
            _ => false
       }
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

