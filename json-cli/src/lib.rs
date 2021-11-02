use std::env;
use std::fs;
use std::time::Instant;
use tiny_json as json;

fn print_help_info() {
    println!("Usage: tiny-json <options> filename");
    println!("example: tiny-json -f a.tiny-json\n");
    println!("Options");
    println!("    -f [indent]   format tiny-json file");
    println!("    -m            minify tiny-json file\n");
}

fn format_json(filename: &str, indent: u32) {
    let start = Instant::now();
    let content = fs::read_to_string(filename)
        .expect(&format!("{}{}", "Read file error at:", filename));
    let node = json::parse(&content);
    let format_content = json::stringify(&node, indent);
    fs::write(filename, &format_content).expect("Write File Error");
    println!("Complete! Cost: {:?}", start.elapsed());
}

pub fn json_commander() {
    let mut i = 0;
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    if args.len() == 0 {
        return print_help_info();
    }

    let mut args = args.iter();
    let op = args.next().unwrap();
    if op == "-f" {
        let indent = args.next().unwrap().parse::<i32>().expect("The argument indent should be a number");
        assert!(indent > 0, "The indent should greater than 0");
        let filename = args.next().expect("Missing argument filename");
        format_json(filename, indent as u32);
    } else if op == "-m" {
        let filename = args.next().expect("Missing argument filename");
        format_json(filename, 0);
    } else {
        print_help_info();
    }
}