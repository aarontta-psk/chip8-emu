mod render;
mod core;

fn main() {
    core::overlay::print_overlay();

    let _emu: core::interpreter::Chip8Registry = core::interpreter::Chip8Registry::new();
    //     let contents = fs::read("Airplane.ch8").expect("Should have been able to read the file");
    //     println!("{}\n{:#04x?}", msg, contents);

    //     let file = fs::read_to_string("instr.txt").expect("Should have been able to read the file");
    //     let instr: Vec<&str> = file.split("\r\n\r\n").collect();
    //     println!("{:?}", instr);
    //     process_input(&contents, &instr);

    //     // let emu: Chip8Registry = Chip8Registry::new();
    //     // println!("{:?} {} {} {} {} {} {:?}", emu.vx, emu.i, emu.pc, emu.sp, emu.dt, emu.st, emu.stack);

    //     let _app = winit::App::new();
}