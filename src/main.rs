
use std::env;
use std::path::Path;
use chip_8::cpu::Cpu;


mod chip_8;

const ROM_PATH :&'static str = "D:\\DATA\\Programming\\Rust\\rust8\\roms";

fn main() {
    
    let mut cpu = Cpu::new();

    cpu.init();
    
    let rom_name = env::args().nth(1).unwrap();
    
    cpu.load_rom(&rom_name, Path::new(ROM_PATH));

    println!("{:?}", cpu.mem);

}









