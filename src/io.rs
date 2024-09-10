use alloc::vec::Vec;
use alloc::string::String;

pub struct Stdin {
    stdin_buff: Vec<char>,
}

impl Stdin {
    pub fn write(&mut self, character: char) {
        if character != '\n' {
            self.stdin_buff.push(character);
        }
    }

    pub fn read(&self) -> String {
        let mut buff = String::new();
        self.stdin_buff.iter().for_each(|&c| buff.push(c));
        buff
    }

    pub fn clear(&mut self) {
        self.stdin_buff.clear();
    }
}

pub fn stdin_new(stdin_vec: Vec<char>) -> Stdin {
    Stdin {
        stdin_buff: stdin_vec,
    }
}
