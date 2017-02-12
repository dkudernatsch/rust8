extern crate bit_vec;
extern crate piston_window;

use std::env;
use std::path::Path;
use std::io;

use chip_8::cpu::Cpu;
use piston_window::*;



mod chip_8;

const ROM_PATH :&'static str = "D:\\DATA\\Programming\\Rust\\rust8\\roms";

fn main() {

    let mut window: PistonWindow = WindowSettings::new("Hello Rust", [640, 320])
        .exit_on_esc(false)
        .build()
        .unwrap_or_else(|e| panic!("failed to build window: {:?}", e));



        while let Some(e) = window.next() {

            if let Some(args) = e.press_args() {
                println!("Press: {:?}", args);
            }

            if let Some(args) = e.release_args() {
                println!("Release: {:?}", args);
            }
        }
    
/*
    let mut cpu = Cpu::new();
    cpu.init();

    let sprite = &[0xFF];

    cpu.display.clear();
    cpu.display.draw(sprite, 1, 2);
    cpu.display.draw(sprite, 4, 2);

    println!("{:?}", cpu.display);
*/
    /*
    let mut cpu = Cpu::new();

    cpu.init();
    
    //cpu.display.draw_sprite(&[0xEE], 1,1);
    //println!("{:?}", cpu.display);
    let rom_name = env::args().nth(1).unwrap();
    
    cpu.load_rom(&rom_name, Path::new(ROM_PATH));

    //println!("{:?}", cpu.mem);
    cpu.display.clear();
    //print!("{:?}", cpu.display);

    while cpu.step().is_ok() {
        let mut buf = String::new();
        if cpu.mem.read_word(cpu.program_counter-2) >> 12 == 0xd as u16 {
            
            println!("{:04X}", cpu.mem.read_word(cpu.program_counter-2));
            println!("{:?}", cpu.display);
            //println!("{:?}", cpu.mem);
            //println!("{:?}", cpu);

            io::stdin().read_line(&mut buf).ok();
        }

        
        
    }*/
    
}









