use std::io::stdin;
use ureq::*;

fn main() {
    let mut client = Client {
        agent: ureq::AgentBuilder::new().build(),
        base_url: String::from("https://api.dnsdb.info/"),
        input_string: String::new(),
        input_history: Vec::new(),
        API_KEY: None
    };
    let ir = stdin();
    loop {
        ir.read_line(&mut client.input_string);
        client.input_history.push(client.input_string.clone());
        client.handle_input(client.input_string.split_whitespace().collect::<_>());
    }
}

pub(crate) struct Client {
    agent: ureq::Agent,
    base_url: String,
    input_string: String,
    input_history: Vec<String>,
    API_KEY: Option<String>
}impl Client {
    
pub fn handle_input(&mut self, tokens: Vec<String>) {
    match &tokens[0] {
        "ping" => {
            self.ping();
            return;
        }
        "set" => {
            self.set(&tokens[1..]);
            return;
        }
        _ => {
            println!("Unrecognized token: \"{token}\".", token =tokens[0]);
            return;
        }
    }

}

fn ping(&mut self) {
            let response = self.agent.get(&self.base_url + String::from("dnsdb/v2/ping"));
            println!("Response: {response}.", response=response.to_string());
        }

fn set(&self, input: &[String]) {
    match input[0] {
        "API_KEY" => {
            self.API_KEY = Some(input[1].clone());
            return;
        }
        _ => {
            println!("Error: token \"{token}\" is invalid.", token=input[0]);
        }
    }
}
}