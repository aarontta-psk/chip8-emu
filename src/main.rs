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

fn main() {
    let emu: Chip8Registry = Chip8Registry::new();
    println!("{:?} {} {} {} {} {} {:?}", emu.vx, emu.i, emu.pc, emu.sp, emu.dt, emu.st, emu.stack);
}
