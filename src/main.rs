use std::fs;

mod chip8_numbers {
    pub const Zero: [u8; 5] = [0xF0, 0x90, 0x90, 0x90, 0xF0];
    pub const One: [u8; 5] = [0x20, 0x60, 0x20, 0x20, 0x70];
    pub const Two: [u8; 5] = [0xF0, 0x10, 0xF0, 0x80, 0xF0];
    pub const Three: [u8; 5] = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
    pub const Four: [u8; 5] = [0x90, 0x90, 0xF0, 0x10, 0x10];
    pub const Five: [u8; 5] = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
    pub const Six: [u8; 5] = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
    pub const Seven: [u8; 5] = [0xF0, 0x10, 0x20, 0x40, 0x40];
    pub const Eight: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0xF0];
    pub const Nine: [u8; 5] = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
    pub const LetterA: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0x90];
    pub const LetterB: [u8; 5] = [0xE0, 0x90, 0xE0, 0x90, 0xE0];
    pub const LetterC: [u8; 5] = [0xF0, 0x80, 0x80, 0x80, 0xF0];
    pub const LetterD: [u8; 5] = [0xE0, 0x90, 0x90, 0x90, 0xE0];
    pub const LetterE: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
    pub const LetterF: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0x80];
}

// #[derive(Default)]
struct Chip8Registry {
    vx: [u8; 0x10],     // general purpose registries (VF is a flag, do not use it)
    stack: [u16; 0x10], // subroutine stack
    i: u16,     // memory address (only uses rightmost 12 bits)
    pc: u16,    // program counter
    sp: u8,     // stack pointer
    dt: u8,     // delay timer registry
    st: u8,     // sound timer registry
}

impl Chip8Registry {
    fn new() -> Self { //&mut Self
        Self { 
            vx: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            stack: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            i: 0, 
            pc: 1,
            sp: 2,
            dt: 3, 
            st: 4,
        }
    }
}

fn process_instr<'a>(command: &u16, instr: &'a Vec<&str>) -> &'a str {
    let first_num: u8 = ((command & 0xF000) >> 12) as u8;
    let second_num: u8 = ((command & 0xF00) >> 8) as u8;
    let third_num: u8 = ((command & 0xF0) >> 4) as u8;
    let fourth_num: u8 = (command & 0xF) as u8;

    match first_num {
        0x0 => match third_num {
            0xC => return instr[1],
            0xE => {
                if fourth_num == 0x0 {
                    return instr[2];
                } else {
                    return instr[3];
                }
            }
            0xF => return instr[4],
            _ => return instr[0],
        },
        0x1 => return instr[9],
        0x2 => return instr[10],
        0x3 => return instr[11],
        0x4 => return instr[12],
        0x5 => return instr[13],
        0x6 => return instr[14],
        0x7 => return instr[15],
        0x8 => match fourth_num {
            0x0 => return instr[16],
            0x1 => return instr[17],
            0x2 => return instr[18],
            0x3 => return instr[19],
            0x4 => return instr[20],
            0x5 => return instr[21],
            0x6 => return instr[22],
            0x7 => return instr[23],
            0xE => return instr[24],
            _ => return "what",
        },
        0x9 => return instr[25],
        0xA => return instr[26],
        0xB => return instr[27],
        0xC => return instr[28],
        0xD => {
            if fourth_num == 0x0 {
                return instr[29];
            } else {
                return instr[30];
            }
        }
        0xE => {
            if third_num == 0x9 {
                return instr[31];
            } else {
                return instr[32];
            }
        }
        0xF => return instr[33],
        _ => return "what",
    }
}

fn process_input(contents: &Vec<u8>, instr: &Vec<&str>) -> () {
    let mut command: u16 = 0;
    let mut count: u16 = 0;
    let mut flag: bool = true;

    for elem in contents.iter() {
        if flag {
            command = (*elem as u16) << 8;
        } else {
            command += *elem as u16;
            println!(
                "Instruction {}: {:#06x} -> {}",
                count / 2,
                command,
                process_instr(&command, instr)
            );
            command = 0;
        }
        count += 1;
        flag = !flag;
    }
}

fn main() {
    let msg: String = String::from("Chip 8 Emulator!");
    let contents = fs::read("Airplane.ch8").expect("Should have been able to read the file");
    println!("{}\n{:#04x?}", msg, contents);

    let file = fs::read_to_string("instr.txt").expect("Should have been able to read the file");
    let instr: Vec<&str> = file.split("\r\n\r\n").collect();
    println!("{:?}", instr);

    process_input(&contents, &instr);
   
    // let emu: Chip8Registry = Chip8Registry::new();
    // println!("{:?} {} {} {} {} {} {:?}", emu.vx, emu.i, emu.pc, emu.sp, emu.dt, emu.st, emu.stack);
}
