use icon_enums::{create_enum_file};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: icon_enums <input_dir> <output_file>");
        std::process::exit(1);
    }

    let input_dir = &args[1];
    let output_file = &args[2];

    match create_enum_file(input_dir, output_file) {
        Ok(_) => println!("Enum file created: {}", output_file),
        Err(e) => eprintln!("Error: {}", e),
    }
}
