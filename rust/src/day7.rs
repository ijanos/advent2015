use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::collections::HashMap;


#[derive(Debug, Clone)]
enum Wire {
    Named(String),
    Signal(u32),
}

#[derive(Debug, Clone)]
enum Connection {
    BinaryGate {
        op: Operation,
        a: Wire,
        b: Wire,
    },
    NOT(Wire),
    Direct(Wire),
}

#[derive(Debug, Clone)]
enum Operation {
    AND,
    OR,
    LSHIFT,
    RSHIFT,
}

struct Circuit {
    connections: HashMap<String, Connection>,
    cache: HashMap<String, u32>,
}

impl FromStr for Connection {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Connection, Self::Err> {
        let rule: Vec<&str> = input.split(' ').collect();
        let wire = |s: &str| {
            match s.parse::<u32>() {
                Ok(x) => Wire::Signal(x),
                Err(_) => Wire::Named(s.to_owned()),
            }
        };
        let conn: Connection = match rule.len() {
            1 => Connection::Direct(wire(rule[0])),
            2 => Connection::NOT(Wire::Named(rule[1].to_owned())),
            3 => {
                match rule[1] {
                    "AND" => {
                        Connection::BinaryGate {
                            a: wire(rule[0]),
                            b: wire(rule[2]),
                            op: Operation::AND,
                        }
                    }
                    "OR" => {
                        Connection::BinaryGate {
                            a: wire(rule[0]),
                            b: wire(rule[2]),
                            op: Operation::OR,
                        }
                    }
                    "LSHIFT" => {
                        Connection::BinaryGate {
                            a: wire(rule[0]),
                            b: wire(rule[2]),
                            op: Operation::LSHIFT,
                        }
                    }
                    "RSHIFT" => {
                        Connection::BinaryGate {
                            a: wire(rule[0]),
                            b: wire(rule[2]),
                            op: Operation::RSHIFT,
                        }
                    }
                    _ => return Err("invalid operation"),
                }
            }
            _ => return Err("invalid connections string"),
        };
        Ok(conn)
    }
}

impl FromStr for Circuit {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Circuit, Self::Err> {
        let mut curcuit = HashMap::new();
        for line in input.lines() {
            let rule: Vec<&str> = line.split(" -> ").collect();
            let conn = try!(rule[0].parse::<Connection>());
            curcuit.insert(rule[1].to_owned(), conn);
        }
        Ok(Circuit {
            connections: curcuit,
            cache: HashMap::new(),
        })
    }
}

impl Circuit {
    fn eval(&mut self, id: &str) -> u32 {
        if let Some(v) = self.cache.get(id) {
            return *v;
        }
        let connection = self.connections.get(id).unwrap().clone();
        let value = {
            let mut unwire = |w: &Wire| {
                match *w {
                    Wire::Signal(a) => a,
                    Wire::Named(ref name) => self.eval(name),
                }
            };
            match connection {
                Connection::Direct(ref wire) => unwire(wire),
                Connection::NOT(ref wire) => !unwire(wire),
                Connection::BinaryGate{ref op, ref a, ref b} => {
                    match *op {
                        Operation::AND => unwire(a) & unwire(b),
                        Operation::OR => unwire(a) | unwire(b),
                        Operation::LSHIFT => unwire(a) << unwire(b),
                        Operation::RSHIFT => unwire(a) >> unwire(b),
                    }
                }
            }
        };
        self.cache.insert(id.to_owned(), value);
        value
    }
}

pub fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).ok();
    let mut circuit = buffer.parse::<Circuit>().unwrap();
    println!("{}", circuit.eval("a"));
}
