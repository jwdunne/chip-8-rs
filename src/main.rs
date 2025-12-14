use std::{env, fs::read, process};

use minifb::{Key, Window, WindowOptions};

const WHITE: u32 = 0xFFFFFF;
const BLACK: u32 = 0x000000;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 10;

const PROGRAM_START: u16 = 0x200;

const FONT_SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x90, 0x90, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

struct Chip8 {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    stack: [u8; 64],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    display: [bool; WIDTH * HEIGHT],
    keypad: [bool; 16],
}

impl Chip8 {
    fn new(program: &[u8]) -> Self {
        let mut machine = Self {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            stack: [0; 64],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [false; WIDTH * HEIGHT],
            keypad: [false; 16],
        };

        let offset = 0x50;
        for (i, &piece) in FONT_SPRITES.iter().enumerate() {
            machine.memory[offset + i] = piece;
        }

        for (i, &byte) in program.iter().enumerate() {
            machine.memory[PROGRAM_START as usize + i] = byte;
        }

        machine
    }
}

fn main() {
    // Initialisation

    let bin = env::args().nth(0).unwrap();

    let rom_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: {bin} <rom>");
            process::exit(1);
        }
    };

    println!("Running: {bin} {rom_path}");

    let program = read(rom_path).expect("could not read ROM file");
    let _machine = Chip8::new(&program);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    for i in 0..(WIDTH * HEIGHT) {
        buffer[i] = if i % 2 == 0 { BLACK } else { WHITE }
    }

    let mut window = Window::new(
        "CHIP-8",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Failed to update window");
    }
}
