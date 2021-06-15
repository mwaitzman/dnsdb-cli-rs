//use std::io::stdin;
extern crate ureq;
//extern crate serde;
//extern crate serde_json;
#[allow(unused_imports)]//tired of getting warns for this while prototyping...
use ureq::*;
use std::time::Duration;
use std::env;
fn main() {
    let base_url = "https://api.dnsdb.info/";
    let args: Vec<String> = env::args().collect();
    #[allow(non_snake_case)]
    let mut API_Key: Option<String> = None;
    let arg = &args[1][..];
    match arg {
        "ping" => {
            let response = ureq::get(&(base_url.to_owned()  + "dnsdb/v2/ping"))
                .timeout(Duration::new(60,0))
                .call();
                match response {
                    Ok(response) => {
                        println!("Response: {}", 
                        match response.into_string() {
                            Ok(s) => s,
                            _ => todo!()
                        }
                    );
                    }
                    _ => {
                        todo!();
                    }
                }
        },
        "rate_limit" => {
            for i in 2..args.len() {
                if &args[i][0..6] == "--key=" {
                    API_Key = Some(String::from(&args[i][6..]));
                    commands::rate_limit(&API_Key.unwrap());
                }
            }
        },
        _ => todo!()
    }
}
pub mod commands {
    pub fn rate_limit(key: &String) {
        let response = ureq::get(&"https://api.dnsdb.info/dnsdb/v2/rate_limit")
            .set("X-API-Key", key)
            .call();
        match response {
            Ok(response) => {
                println!("Response: {}", 
                    match response.into_string() {
                        Ok(s) => s,
                        _ => todo!()
                    }
                );
            }
            _ => {
                todo!();
            }
        }
    }
}
//use serde::{Deserialize, Serialize};
// fn main() {
//     let mut input_string = String::new();
//     let mut client = Client {
//         agent: ureq::AgentBuilder::new().build(),
//         base_url: String::from("https://api.dnsdb.info/"),
//         input_history: Vec::new(),
//         API_KEY: None,
//     };
//     let ir = stdin();
//     loop {
//         ir.read_line(&mut input_string);
//         client.input_history.push(input_string.clone());
//         client.handle_input(input_string.split_whitespace());
//         input_string.clear();
//     }
// }
//
// pub(crate) struct Client {
//     agent: ureq::Agent,
//     base_url: String,
//     input_history: Vec<String>,
//     API_KEY: Option<String>,
// }
// impl Client {
//     pub fn handle_input<'a>(&mut self, mut tokens: impl Iterator<Item = &'a str>) {
//         let token = tokens.next().unwrap();
//         match token {
//             "ping" => {
//                 self.ping();
//                 return;
//             }
//             "set" => {
//                 self.set(tokens);
//                 return;
//             }
//             "rate_limit" => {
//                 self.rate_limit();
//                 return;
//             }
//             "listcommands" => {
//                 println!("lookup, rate_limit, ping, set");
//             }
//             "help" => {
//                 let token = tokens.next();
//                 match token {
//                     None => {
//                         println!("For a list of commands, type \"listcommands\". For detailed information on a particular command, type \"help <command name>\"");
//                         return;
//                     }
//                     Some(_T) => {
//                         let token = token.unwrap();
//                         match token {
//                             "ping" => {
//                                 println!("Pings the DNSDB database with a packet and prints the time taken to receive a response packet. This command takes no arguments (and in fact does not even require an API key).");
//                                 return;
//                             }
//                             _ => {
//                                 println!("Could not display help for a command named {}. Maybe it doesn't exist?", token);
//                                 return;
//                             }
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 println!("Unrecognized token: \"{token}\".", token = token);
//                 return;
//             }
//         }
//     }
//
//     fn ping(&mut self) {
//         let response = self
//             .agent
//             .get(&(self.base_url.clone() + &String::from("dnsdb/v2/ping")))
//             .timeout(Duration::new(10,0))//will time out after 10 seconds
//             .call();
//         println!("Response: {response:?}.", response = response);
//     }
//
//     fn set<'a>(&mut self, mut input: impl Iterator<Item = &'a str>) {
//         let token = input.next().unwrap();
//         match token {
//             "API_KEY" => {
//                 self.API_KEY = Some(input.next().unwrap().to_owned());
//                 return;
//             }
//             _ => {
//                 println!("Error: token \"{token}\" is invalid.", token = token);
//             }
//         }
//     }
//
//     fn rate_limit(&mut self) {
//         todo!();
//     }
// }
/*pub mod response_stuff {
    enum states {
        null,
        initiating,
        ongoing,
        terminating
    }
    #[derive(Serialize, Deserialize)]
    struct response_struct {
    
    }
    impl response_struct {
        pub fn from_str_vec(input: Vec<str>) -> response_struct {
            response_struct::from_str_iter(input.into_iter())
        }
        pub fn from_str_iter(input : Iter<str>) -> response_struct {
            let state = states::null;
            for line in input {
                match state {
                    null => {
                        match line {

                        }
                    }
                }
            }
        }
    }

    enum possible_values {
        cond_enum,
        empty_brackets,

    }
    enum cond_enum {
        begin,
        ongoing,
        succeeded,
        limited(String),
        failed(String)
    }
}*/

/*
let matches = App::new("My Super Program")
                          .version("1.0")
                          .author("Kevin K. <kbknapp@gmail.com>")
                          .about("Does awesome things")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .takes_value(true))
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("1.3")
                                      .author("Someone E. <someone_else@other.com>")
                                      .arg(Arg::with_name("debug")
                                          .short("d")
                                          .help("print debug information verbosely")))
                          .get_matches();
*/