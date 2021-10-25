use std::fs;
use tiny_json as json;

fn main() {
    println!("\n=============================== OUTPUT ======================================\n");
    let content = fs::read_to_string("test.txt").unwrap();
    let node = json::parse(content.as_str());

    json::walk(&node, |current| {
        println!("{}", current.to_string());
    });

    let result = json::stringify(&node, 4);
    println!("STRINGIFY:\n{}", result);

    // println!("\n=============================== CLI ======================================\n");
    // json-cli::json_commander();
}
