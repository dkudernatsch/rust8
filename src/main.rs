
use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use chip_8::cpu::Cpu;
use chip_8::cpu::Mem;
use chip_8::cpu::Instruction;
use chip_8::io::Display;

mod chip_8;

const ROM_PATH :&'static str = "D:\\DATA\\Programming\\Rust\\rust8\\roms";

fn main() {
    let mut cpu = Cpu::new();
    let mut mem = Mem::new();
    let d = Display::new();

    init(&mut cpu, &mut mem);
    
    let rom_name = env::args().nth(1).unwrap();
    let rom_buf = load_rom(&rom_name, Path::new(ROM_PATH)).unwrap();

    match mem.load(&rom_buf, 0x200){
        Ok(_) => (),
        Err(x) => panic!("Failed to load rom into ram: {:?}", x)
    }

    print!("{:?}", mem);

    let instruction = mem.read_word(cpu.program_counter);
    println!("{:04X}", instruction);
    
    let n = Instruction{bytes:&[0xAA, 0x1E]};

    println!("{:?}", n);
    println!("{:?} {:?} {:?} {:?}", n.at(0), n.at(1), n.at(2), n.at(3));

}

fn init<'a>(cpu: &'a mut Cpu, mem: &'a mut Mem) -> Result<(), &'a str>{
    // load characters 0-F into memory starting at 0x100 end 0x140
    try!(load_characters(mem));
    cpu.program_counter = 0x200;
    Ok(())
}

fn load_characters(mem: &mut Mem) -> Result<(),&str>{
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
    mem.load(buf, c)
}

fn load_rom<'a>(name: &String, rom_path: &Path) -> Result<Vec<u8>, &'a str>{
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
                Ok(_) => Ok(buf),
                Err(_) => Err("Could not read file")
            }
        },
        Err(x) => Err(x),
    }
}



