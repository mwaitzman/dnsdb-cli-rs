use std::io::stdin;
use ureq::*;

fn main() {
    let mut input_string = String::new();
    let mut client = Client {
        agent: ureq::AgentBuilder::new().build(),
        base_url: String::from("https://api.dnsdb.info/"),
        input_history: Vec::new(),
        API_KEY: None,
    };
    let ir = stdin();
    loop {
        ir.read_line(&mut input_string);
        client.input_history.push(input_string.clone());
        client.handle_input(input_string.split_whitespace());
        input_string.clear();
    }
}

pub(crate) struct Client {
    agent: ureq::Agent,
    base_url: String,
    input_history: Vec<String>,
    API_KEY: Option<String>,
}
impl Client {
    pub fn handle_input<'a>(&mut self, mut tokens: impl Iterator<Item = &'a str>) {
        let token = tokens.next().unwrap();
        match token {
            "ping" => {
                self.ping();
                return;
            }
            "set" => {
                self.set(tokens);
                return;
            }
            "rate_limit" => {
                self.rate_limit();
                return;
            }
            "listcommands" => {
                println!("lookup, rate_limit, ping, set");
            }
            "help" => {
                let token = tokens.next();
                match token {
                    None => {
                        println!("For a list of commands, type \"listcommands\". For detailed information on a particular command, type \"help <command name>\"");
                        return;
                    }
                    Some(_T) => {
                        let token = token.unwrap();
                        match token {
                            "ping" => {
                                println!("Pings the DNSDB database with a packet and prints the time taken to receive a response packet. This command takes no arguments (and in fact does not even require an API key).");
                                return;
                            }
                            _ => {
                                println!("Could not display help for a command named {}. Maybe it doesn't exist?", token);
                                return;
                            }
                        }
                    }
                }
            }
            _ => {
                println!("Unrecognized token: \"{token}\".", token = token);
                return;
            }
        }
    }

    fn ping(&mut self) {
        let response = self
            .agent
            .get(&(self.base_url.clone() + &String::from("dnsdb/v2/ping")));
        println!("Response: {response:?}.", response = response);
    }

    fn set<'a>(&mut self, mut input: impl Iterator<Item = &'a str>) {
        let token = input.next().unwrap();
        match token {
            "API_KEY" => {
                self.API_KEY = Some(input.next().unwrap().to_owned());
                return;
            }
            _ => {
                println!("Error: token \"{token}\" is invalid.", token = token);
            }
        }
    }

    fn rate_limit(&mut self) {
        todo!();
    }
}