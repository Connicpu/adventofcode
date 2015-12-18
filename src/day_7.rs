use std::fs;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Value {
    Constant(u16),
    Wire(String),
    And(Box<Value>, Box<Value>),
    Or(Box<Value>, Box<Value>),
    LShift(Box<Value>, Box<Value>),
    RShift(Box<Value>, Box<Value>),
    Not(Box<Value>),
}

impl Value {
    fn evaluate(&self, state: &mut State) -> u16 {
        use self::Value::*;
        match *self {
            Constant(c) => c,
            Wire(ref name) => state.evaluate(name),
            And(ref left, ref right) => left.evaluate(state) & right.evaluate(state),
            Or(ref left, ref right) => left.evaluate(state) | right.evaluate(state),
            LShift(ref left, ref right) => left.evaluate(state) << right.evaluate(state),
            RShift(ref left, ref right) => left.evaluate(state) >> right.evaluate(state),
            Not(ref value) => !value.evaluate(state)
        }
    }
}

struct State {
    wires: HashMap<String, Value>,
    results: HashMap<String, u16>,
}

impl State {
    fn new() -> State {
        State {
            wires: HashMap::new(),
            results: HashMap::new(),
        }
    }
    
    fn set(&mut self, wire: String, value: Value) {
        self.wires.insert(wire, value);
    }
    
    fn evaluate(&mut self, name: &str) -> u16 {
        if self.results.contains_key(name) {
            self.results[name]
        } else {
            let value = self.wires[name].clone();
            let result = value.evaluate(self);
            self.results.insert(name.into(), result);
            result
        }
    }
    
    fn clear(&mut self) {
        self.results.clear();
    }
}

fn parse_operand(operand: &str) -> Value {
    match str::parse::<u16>(operand) {
        Ok(c) => Value::Constant(c),
        _ => Value::Wire(operand.into()),
    }
}

fn bad_line(line: &str) -> ! {
    panic!("Invalid instruction `{}`", line)
}

fn parse_line(line: &str, state: &mut State) {
    let mut split = line.split_whitespace().peekable();
    
    let value = match split.next() {
        Some("NOT") => match split.next() {
            Some(operand) => Value::Not(Box::new(parse_operand(operand))),
            _ => bad_line(line),
        },
        Some(left) => match (parse_operand(left), split.peek()) {
            (left, Some(&"->")) => left, 
            (left, _) => match (split.next(), parse_operand(split.next().unwrap())) {
                (Some("AND"), right) => Value::And(Box::new(left), Box::new(right)),
                (Some("OR"), right) => Value::Or(Box::new(left), Box::new(right)),
                (Some("LSHIFT"), right) => Value::LShift(Box::new(left), Box::new(right)),
                (Some("RSHIFT"), right) => Value::RShift(Box::new(left), Box::new(right)),
                _ => bad_line(line),
            }
        },
        _ => bad_line(line),
    };
    
    assert_eq!(split.next(), Some("->"));
    let wire = split.next().unwrap().into();
    
    state.set(wire, value);
}

pub fn puzzle_1() {
    let mut state = State::new();
    let file = fs::File::open("./input/day7.txt").unwrap();
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line.unwrap();
        parse_line(&line, &mut state);
    }
    
    let result = state.evaluate("a");
    println!("a = {}", result);
    
    println!("b := a, reset");
    state.clear();
    state.set("b".into(), Value::Constant(result));
    
    let result = state.evaluate("a");
    println!("a = {}", result);
}
