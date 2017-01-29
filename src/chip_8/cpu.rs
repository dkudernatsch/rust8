
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Write;
use std::fmt;

#[derive(Debug)]
pub struct Cpu {
    register: [u8; 16],

    pub sound_timer: u8,
    pub delay_timer: u8,

    pub program_counter: u16,

    pub stack_pointer: u8,
    stack: [u8; 16],
}

impl Cpu {
    pub fn new() -> Cpu{
        Cpu{
            register: [0; 16],
            sound_timer: 0,
            delay_timer: 0,
            program_counter: 0,
            stack_pointer: 0,
            stack: [0; 16]
        }
    }
}



pub struct Mem {
    data: [u8; 4096]
}

impl Debug for Mem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        let mut c = 0;
        let mut string = String::new();
        while c < self.data.len() {
            let slice: &[u8] = &self.data[c..c+16];
            writeln!(&mut string, "{:04X}:  {:02X}  {:02X}  {:02X}  {:02X}   {:02X}  {:02X}  {:02X}  {:02X}   {:02X}  {:02X}  {:02X}  {:02X}   {:02X}  {:02X}  {:02X}  {:02X}", 
            c ,slice[0],slice[1],slice[2],slice[3],slice[4],slice[5],slice[6],slice[7],slice[8],slice[9],slice[10],slice[11],slice[12],slice[13],slice[14],slice[15]
            ).unwrap();
            c = c + 16;
        }
        write!(f, "{}", string)
    }
}

impl Mem{
    pub fn new() -> Mem{
        Mem{data:[0; 4096]}
    }

    pub fn load<'a, 'b>(&'a mut self, buf: &Vec<u8>, pos: u16) -> Result<(),&'b str>{
        let last = pos + buf.len() as u16;

        if last > self.data.len() as u16 {
            return Err("Overflow Error");
        }else{
            let slice: &mut[u8] = &mut self.data[pos as usize..last as usize];
            slice.clone_from_slice(buf.as_slice());
            Ok(())
        }
    }

    pub fn read_byte(&self, address :u16)->u8{
        if self.data.len() >= address as usize{
            self.data[address as usize]
        }else {
            panic!("Overflow trying to read address: {}",address);
        }
    }

    pub fn read_word(&self, address :u16)->u16{
        ((self.read_byte(address) as u16) << 8) + self.read_byte(address +1) as u16
    }
    
}


#[derive(Debug)]
enum Instruction {
   
}