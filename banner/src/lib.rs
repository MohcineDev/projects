use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        let n: Vec<char> = name.chars().collect();

        let mut short = String::from("-");
        short.push(n[0]);

        let mut long = String::from("--");
        long.push_str(name);

        return Flag {
            short_hand: short,
            long_hand: long,
            desc: String::from(d),
        };
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(String::from(flag.short_hand), func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
    
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a: Callback;
       // let res = a(input, argv.to_vec().concat().as_str());
        res
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
     let a: Callback;
  //      let res = a(input, argv.to_vec().concat().as_str());
        res
}
