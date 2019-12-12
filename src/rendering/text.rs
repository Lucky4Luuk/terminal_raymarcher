//WIP
use std::collections::HashMap;

const characters: &str = "abcdefghijklmnopqrstuvwxyz";

pub struct Font {
    pub data: HashMap<String, String>,
}

impl Font {
    pub fn new() -> Font {
        Font {
            data: HashMap::new(),
        }
    }

    pub fn load(&mut self) {
        //For now, the file and characters are hardcoded
        
    }
}
