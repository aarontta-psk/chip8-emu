#[derive(Default)]
pub struct Chip8Registry {
    vx: [u8; 0x10],     // general purpose registries (VF is a flag, do not use it)
    stack: [u16; 0x10], // subroutine stack
    i: u16,     // memory address (only uses rightmost 12 bits)
    pc: u16,    // program counter
    sp: u8,     // stack pointer
    dt: u8,     // delay timer registry
    st: u8,     // sound timer registry
}

impl Chip8Registry {
    pub fn new() -> Self { //&mut Self
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

pub fn process_instr<'a>(command: &u16, instr: &'a Vec<&str>) -> &'a str {
    let first_num: u8 = ((command & 0xF000) >> 12) as u8;
    let _second_num: u8 = ((command & 0xF00) >> 8) as u8;
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

pub fn process_input(contents: &Vec<u8>, instr: &Vec<&str>) -> () {
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