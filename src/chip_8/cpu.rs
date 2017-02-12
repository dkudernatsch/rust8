
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Write;
use std::fmt;

use std::path::PathBuf;
use std::path::Path;

use std::fs::File;

use std::io::Read;

use std::collections::HashMap; 

use chip_8::io::Display;
use chip_8::io::Keyboard;

pub struct Cpu {
    
    pub mem: Mem,
    pub display: Display,
    pub keyboard: Keyboard,

    pub sound_timer: u8,
    pub delay_timer: u8,

    pub program_counter: u16,

    reg_i: u16,

    pub stack_pointer: u8,
    stack: [u16; 16],
    register: [u8; 16]
}
 
impl Debug for Cpu {
     fn fmt(&self, f: &mut Formatter) -> fmt::Result{
         let mut buf = String::new();
         //writeln!(f, "sound_timer: {:?}", self.sound_timer);
         //writeln!(f, "delay_timer: {:?}", self.delay_timer);
         writeln!(buf, "P_C  : {:04X}", self.program_counter);
         writeln!(buf, "stack_pointer: {:?}", self.stack_pointer);
         writeln!(buf, "stack: {:#?}", self.stack);
         for (i, item) in self.register.iter().enumerate(){
            writeln!(buf, "{:01X} : {:02X}",i,item);
         }
         writeln!(buf, "reg_i: {:04X}", self.reg_i);
         writeln!(f, "{}", buf)
     }
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

    pub fn step(&mut self) -> Result<(),&str>{
        let ins_code = self.mem.read_word(self.program_counter);
        println!("excuting: {:04X}", ins_code);
        let instruction: Option<&Instruction> = Instruction::get_instruction(ins_code);
        println!("{:?}", instruction);
        let ret = match instruction {
            Some(ins) => {
                let f = ins.action;
                f(ins_code, self);
                Ok(())
            },
            None => panic!("unkown instruction: {:04X}", ins_code)
        };
    
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    
        ret
    }

    fn next(&mut self){
        self.program_counter += 2;
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

    pub fn read(&self, start: u16, length: usize) -> &[u8] {
        &self.data[start as usize..start as usize +length]
    }

    pub fn write_byte(&mut self, b: u8, address: u16){
        self.data[address as usize] = b;
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


const INSTRUCTIONS: &'static [Instruction<'static>] = &[
    Instruction{input_mask: 0xFFFF, opcode: 0x00E0, action: &|ins, cpu|{                        // CLS
        cpu.display.clear();
        cpu.next();
    }},  
    Instruction{input_mask: 0xFFFF, opcode: 0x00EE, action: &|ins, cpu|{                         // RET
        cpu.stack_pointer -= 1;
        cpu.program_counter = cpu.stack[cpu.stack_pointer as usize];
        cpu.next();
    }},                  
    Instruction{input_mask: 0xF000, opcode: 0x1000, action: &|ins, cpu|{                        // JMP nnn
        cpu.program_counter = ins & 0x0FFF;
    }},
    Instruction{input_mask: 0xF000, opcode: 0x2000, action: &|ins, cpu|{                        // CALL, addr
        cpu.stack[cpu.stack_pointer as usize] = cpu.program_counter;
        cpu.stack_pointer += 1;
        cpu.program_counter = ins & 0x0F_FF; 
    }},
    Instruction{input_mask: 0xF000, opcode: 0x3000, action: &|ins, cpu|{                        // SE, Vx, byte
        let reg = Instruction::get_nibble(ins, 2) as usize;
        if cpu.register[reg] == (ins & 0x00FF) as u8 {
           cpu.next();
           cpu.next();             
        }else{
            cpu.next();
        }
    }},
    Instruction{input_mask: 0xF000, opcode: 0x6000, action: &|ins, cpu|{                        // LD, Vx, byte
        let reg = Instruction::get_nibble(ins, 2);
        cpu.register[reg as usize] = (ins & 0xFF) as u8;
        cpu.next();
    }},
    Instruction{input_mask: 0xF000, opcode: 0x7000, action: &|ins, cpu|{                        // ADD, Vx, byte
    let reg = Instruction::get_nibble(ins, 2) as usize;
    println!("reg_: {:?}", reg);
        let v = cpu.register[reg];
        cpu.register[reg] = v.wrapping_add((ins & 0x00_FF) as u8);
        cpu.next();
    }},
    Instruction{input_mask: 0xF000, opcode: 0x8000, action: &|ins, cpu|{                        // LD, Vx, Vy
        let reg1 = Instruction::get_nibble(ins, 2);
        let reg2 = Instruction::get_nibble(ins, 1);
        cpu.register[reg1 as usize] = cpu.register[reg2 as usize];
        cpu.next();                
    }},
    Instruction{input_mask: 0xF000, opcode: 0xA000, action: &|ins, cpu|{                        // LD, I, nnn
        cpu.reg_i = ins & 0x0FFF;
        cpu.next();
    }},
    Instruction{input_mask: 0xF000, opcode: 0xD000, action: &|ins, cpu|{                        // DRW, Vx, Vy, nibble
        {let sprite = cpu.mem.read(cpu.reg_i, (ins & 0x00_0F) as usize);
        println!("trying to draw: {:?}", sprite );
        let v_reg_x = cpu.register[Instruction::get_nibble(ins, 2) as usize];
        let v_reg_y = cpu.register[Instruction::get_nibble(ins, 1) as usize];
        if cpu.display.draw(sprite, v_reg_x, v_reg_y){
            cpu.register[15] = 1;
        }else{
            cpu.register[15] = 0;            
        }}
        cpu.next();
    }},
    Instruction{input_mask: 0xF0FF, opcode: 0xF029, action: &|ins, cpu|{                        // Digit sprite
        let v = cpu.register[Instruction::get_nibble(ins, 2) as usize];
        if v > 15 {panic!("requested sprite character bigger than F: {:02X}", v);}
        cpu.reg_i = 5 as u16 * v as u16 + 0x100;
        cpu.next();
    }},    
    Instruction{input_mask: 0xF0FF, opcode: 0xF033, action: &|ins, cpu|{                        // BCD
        let reg = Instruction::get_nibble(ins, 2) as usize;
        let v = cpu.register[reg];
        cpu.mem.write_byte(v / 100,      cpu.reg_i);
        cpu.mem.write_byte((v / 10) %10, cpu.reg_i + 1 as u16 );
        cpu.mem.write_byte(v % 10,       cpu.reg_i + 2 as u16);
        cpu.next();        
    }},        
    Instruction{input_mask: 0xF0FF, opcode: 0xF055, action: &|ins, cpu|{                        // LD [I], Vx
        let reg = Instruction::get_nibble(ins, 2);
        let buf = cpu.register[0..reg as usize].to_vec();
        cpu.mem.load(&buf, cpu.reg_i).ok();
        cpu.next();
    }},
    Instruction{input_mask: 0xF0FF, opcode: 0xF065, action: &|ins, cpu|{                        // LD [I], Vx
        {let reg = Instruction::get_nibble(ins, 2);
        println!("{:02X}", reg);
        println!("{:04X}", cpu.reg_i);
        let mem_slice = cpu.mem.read(cpu.reg_i, (reg+1) as usize);
        let reg_slice = &mut cpu.register[0..(reg+1) as usize];
        reg_slice.clone_from_slice(mem_slice);}
        cpu.next();
    }},        
];


struct Instruction<'a>{
    pub input_mask: u16,
    pub opcode: u16,
    pub action: &'a(Fn(u16, &mut Cpu)->())
}

impl<'a> Debug for Instruction<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        write!(f, "Instruction: {:04X}", self.opcode)
    }
}


impl<'a> Instruction<'a> {
     pub fn get_instruction(code: u16) -> Option<&'a Instruction<'a>>{
         INSTRUCTIONS.into_iter().find(|instruction| code & instruction.input_mask == instruction.opcode)
     }

     pub fn get_u8(ins: u16, pos: u8) -> u8{
        ((ins >> 8 * pos) &0xFF) as u8 
     }

     pub fn get_nibble(ins: u16, pos: u8) -> u8{
        ((ins >> 4*pos) & 0x0F) as u8
     }
}
