use std::{collections::HashMap, num::ParseFloatError};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        println!("flag {:?}", flag);
        self.flags.insert(String::from(flag.short_hand), func);
        self.flags.insert(String::from(flag.long_hand), func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        // pub fn exec_func(&self, input: &str, argv: &[&str]) {
        let first = argv[0];
        let sec = argv[1];
        println!("input : {} ", input);
        if let Some(func) = self.flags.get(input) {
            return match func(first, sec) {
                Ok(v) => Ok(v.to_string()),
                // function not found
                Err(err) => Err(err.to_string()),
            };
        } else {
            Err(String::from("invalid float literal"))
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    // pub fn div(a: &str, b: &str) {
    let num1 = a.parse::<f64>();
    let num2 = b.parse::<f64>();

    // let er = match num1 {
    //     Ok(v) => Ok(v),
    //     Err(err) => Err(err),
    // };

    // let oo = match num2 {
    //     Ok(v) => Ok(v),
    //     Err(err) => Err(err),
    // };
    match (num1, num2) {
        (Ok(n1), Ok(n2)) => {
            if n2 == 0.0 {
               //  return Err(ParseFloatError::from("invalid float literal"));
               // return Err(f64::from("").unwrap_err());   
            }
            Ok((n1 / n2).to_string())
        }
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
    // let res = er.unwrap() / oo.unwrap();
    // Ok(res.to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num1 = a.parse::<f64>();
    let num2 = b.parse::<f64>();

    //chek if parse works
    // let num1_res = match num1 {
    //     Ok(v) => Ok(v),
    //     Err(_) => Err("invalid float literal"),
    // };
    // let num2_res = match num2 {
    //     Ok(v) => Ok(v),
    //     Err(_) => Err("invalid float literal"),
    // };

    match (num1, num2) {
        (Ok(n1), Ok(n2)) => Ok((n1 % n2).to_string()),
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
    // let res = num1_res.unwrap() % num2_res.unwrap();
    // Ok(res.to_string())
}