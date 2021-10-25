use std::fs;
use json::json;

fn main() {
    println!("\n=============================== OUTPUT ======================================\n");
    let content = fs::read_to_string("test.txt").unwrap();
    let node = json::parse(content.as_str());

    // json::walk(&node, |current| {
    //     println!("{}", current.to_string());
    // });

    let result = json::stringify(&node, 5);
    println!("STRINGIFY:\n{}", result);
}
