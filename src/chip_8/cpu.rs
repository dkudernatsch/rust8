
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Write;
use std::fmt;
use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use chip_8::io::Display;
use chip_8::io::Keyboard;

#[derive(Debug)]
pub struct Cpu {
    
    pub mem: Mem,
    pub display: Display,
    pub keyboard: Keyboard,

    pub sound_timer: u8,
    pub delay_timer: u8,

    pub program_counter: u16,

    reg_i: u16,

    pub stack_pointer: u8,
    stack: [u8; 16],
    register: [u8; 16]
}

impl Cpu {
    pub fn new() -> Cpu{
        Cpu{
            mem: Mem::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
            register: [0; 16],
            sound_timer: 0,
            delay_timer: 0,
            reg_i: 0,
            program_counter: 0,
            stack_pointer: 0,
            stack: [0; 16]
        }
    }

    pub fn load_rom<'a>(&mut self, name: &String, rom_path: &Path) -> Result<(), &'a str>{
        let file: Result< PathBuf, &str> = match rom_path.read_dir(){
            Ok(itr) => itr
                        .filter_map(
                            |file_res| match file_res {
                                Ok(file) => Some(file),
                                Err(_) => None})
                        .find(
                            |file| file.file_name().into_string().unwrap_or("".to_string()) == *name
                            )
                        .map(|file| file.path())
                        .ok_or("Could not find file"),
            Err(_) => Err("Could not read directory")
        };
        let mut buf:Vec<u8> = Vec::new();
        
        match file {
            Ok(path) =>{
                let mut file = File::open(path).unwrap();
                match file.read_to_end(&mut buf){
                    Ok(_) => self.mem.load(&buf, 0x200),
                    Err(_) => Err("Could not read file")
                }
            },
            Err(x) => Err(x),
        }
    }

    pub fn init<'a>(&mut self) -> Result<(), &'a str>{
        // load characters 0-F into memory starting at 0x100 end 0x140
        try!(self.load_characters());
        self.program_counter = 0x200;
        Ok(())
    }

    fn load_characters<'a>(&mut self) -> Result<(),&'a str>{
        let c = 0x100;
        let buf = &vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0,
            0x20, 0x60, 0x20, 0x20, 0x70,
            0xF0, 0x10, 0xF0, 0x80, 0xF0,
            0xF0, 0x10, 0xF0, 0x10, 0xF0,
            0x90, 0x90, 0xF0, 0x10, 0x10,
            0xF0, 0x80 ,0xF0 ,0x10 ,0xF0,
            0xF0, 0x80, 0xF0, 0x90, 0xF0,
            0xF0, 0x10, 0x20, 0x40, 0x40,
            0xF0, 0x90, 0xF0, 0x90, 0xF0,
            0xF0, 0x90, 0xF0, 0x10, 0xF0,
            0xF0, 0x90, 0xF0, 0x90, 0x90,
            0xE0, 0x90, 0xE0, 0x90, 0xE0,
            0xF0, 0x80, 0x80, 0x80, 0xF0,
            0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0,
            0xF0, 0x80, 0xF0, 0x80, 0x80
        ];
        self.mem.load(buf, c)
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


