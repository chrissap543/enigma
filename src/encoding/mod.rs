use std::collections::HashMap; 

#[allow(dead_code)]
pub struct Encoder {
    cog1: HashMap<char, char>,
    cog2: HashMap<char, char>,
    cog3: HashMap<char, char>,
    wires: HashMap<char, char>
}

impl Encoder {
    pub fn new(pos1: HashMap<char, char>, pos2: HashMap<char, char>, pos3: HashMap<char, char>, wires: HashMap<char, char>) -> Self {
        Encoder {
            cog1: pos1,
            cog2: pos2,
            cog3: pos3,
            wires
        }
    }

    pub fn encode(&self, message: &String) -> String {
        let mut encoded = String::new(); 

        for c in message.chars() {
            // swap wires if necessary
            let c = Encoder::get_next(&self.wires, &c); 
            // go through first wheel
            let c = Encoder::get_next(&self.cog1, &c); 
            // second wheel
            let c = Encoder::get_next(&self.cog2, &c); 
            // third wheel
            let c = Encoder::get_next(&self.cog3, &c); 
            // back through third
            let c = Encoder::get_next(&self.cog3, &c); 
            // back through second
            let c = Encoder::get_next(&self.cog2, &c);
            // back through first
            let c = Encoder::get_next(&self.cog1, &c); 
            // swap wires if necessary
            let c = Encoder::get_next(&self.wires, &c); 

            encoded.push(c); 
        } 
        encoded
    }

    fn get_next(map: &HashMap<char, char>, c: &char) -> char {
        match map.get(c) {
            Some(next) => *next,
            None => *c
        } 
    }
}