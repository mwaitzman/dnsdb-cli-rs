//use configparser::ini::Ini;
//use std::error::Error;
use std::env;
//use key_handler::*;
//use std::collections::VecDeque;

fn main() {
    println!("Starting program...");

    #[cfg(debug_assertions)]
    {
        for argument in env::args() {
            println!("{}", argument);
        }
    }

    let mut args = helper::sort_args(env::args().collect());
    let api_key = key_handler::do_api_key(&mut args);

    println!("End of program");
}

mod helper {
    //any rust collections with O(1) read(index) ,O(1) write(index), and O(1) delete(index)?
    pub fn sort_args(args: Vec<String>) -> Vec<Vec<String>> {
        enum ArgType {
            Init, //initial state
            CharFlagWithoutParameters,
            CharFlag, ///char flags WITH at least one parameter
            Parameter,
        }
        let mut state = ArgType::Init;
        for arg in args {
            match state {
                ArgType::Init => {
                    if arg.chars().nth(0).unwrap() != '-' {
                        panic!("invalid argument");
                    }
                    unimplemented!();
                }
                _  => unimplemented!(),//TODO
            }
        }
        unimplemented!();
    }
}

mod key_handler {
    pub fn do_api_key(input: &mut Vec<Vec<String>>) -> String {
        unimplemented!();
    }
}
