mod encoding;

use std::collections::HashMap;

fn main() {
    let wires = HashMap::new();
    let mut cog1: HashMap<char, char> = HashMap::new();
    for x in b'a'..=b'z' {
        cog1.insert(x as char, x as char);
    }
    let cog2 = cog1.clone();
    let cog3 = cog1.clone();

    let mut encoder = encoding::Encoder::new(cog1, cog2, cog3, wires);

    let message = String::from("Hello world!");
    println!("Message: {}", message);
    println!("Encoded: {}", encoder.encode(&message));
}
