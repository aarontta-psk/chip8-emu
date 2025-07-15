use colored::Colorize;

pub fn print_overlay() {
    println!("{}\n", "CHIP-8 Emulator".bold().green());
    println!("Please select a ROM from the following:");

    let paths = std::fs::read_dir("./roms").unwrap();
    let files: Vec<std::fs::DirEntry> = paths.filter_map(|dir_entry| {
        return dir_entry.ok();
    }).collect();
    let filenames: Vec<String> = files.iter().filter_map(| file| {
        return file.path().file_name()
            .and_then(|name| 
                name.to_str().map(|name_str| {
                    return name_str.strip_suffix(".ch8").unwrap_or_default().to_string();
                })
            );
    }).collect();
    let mut count: u8 = 1;
    for file in filenames.iter().filter(|item| !item.contains("test")) {
        println!("\t{}. {}", count, file);
        count += 1;
    };

    // let _ = std::io::stdout().flush();

    // let mut s = String::new();
    // std::io::stdin()
    //     .read_line(&mut s)
    //     .expect("Did not enter a correct string");

    // println!("{:?}", files);

    // if let Some('\n') = s.chars().next_back() {
    //     s.pop();
    // }
    // if let Some('\r') = s.chars().next_back() {
    //     s.pop();
    // }
    
    // println!("You typed: {}", s);
}
