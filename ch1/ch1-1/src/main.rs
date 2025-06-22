use std::fs;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let dir_path = if args.len() >= 2 {&args[1]} else {"./"}; 
    let paths = fs::read_dir(dir_path).unwrap();

    for path in paths {
        if let Some(list_file_name) = path.unwrap().path().file_name() {
            println!("{}", list_file_name.to_string_lossy());
        }

    }
}
