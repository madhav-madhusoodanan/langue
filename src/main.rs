use std::io;
use std::fs;
use std::env;
use std::collections::HashMap; 

enum Command {
    SetVar(String, Value),
    GetVar(String)
}

#[derive(Clone, PartialEq, Debug)]
enum Value {
    Nothing,
    Int(i64)
}

#[derive(Debug)]
enum EngineError {
    MissingVariable(String),
    AyyeWrongInstruction,
    LolAreYouInventingNewType,
    GeneratingSomethingFromNothingEh,
    IdkWhatDisIs,
}
struct Evaluator {
    vars: HashMap<String, Value>,
}

impl Evaluator {
    fn new() -> Evaluator {
        Self {
            vars: HashMap::new(),
        }
    }

    fn evaluate(&mut self, commands: &[Command]) -> Result<Value, EngineError> {
        let mut output = Ok(Value::Nothing);
        for command in commands {
            match command {
                Command::SetVar(name, value) => {
                    self.vars.insert(name.into(), value.clone());
                }
                Command::GetVar(name) => {
                    match self.vars.get(name){
                        Some(value) =>  output = Ok(value.clone()),
                        None => return Err(EngineError::MissingVariable(name.into())),
                    }
                }
            }
        }
        output
    }
}

fn parse_set(input: &[&str]) -> Result<Command, EngineError> {
    if input.len() != 3 {
        Err(EngineError::AyyeWrongInstruction)
    }
    else {
        let name = String::from(input[1]);
        let value = match input[2].parse::<i64>() {
            Ok(num) => Value::Int(num),
            Err(_) => return Err(EngineError::LolAreYouInventingNewType)
        };

        Ok(Command::SetVar(name, value))
    }
}

fn parse_get(input: &[&str]) -> Result<Command, EngineError> {
    if input.len() != 2 {
        Err(EngineError::AyyeWrongInstruction)
    }
    else {
        let name = String::from(input[1]);
        Ok(Command::GetVar(name))
    }
}

fn parse(input: &str) -> Result<Vec<Command>, EngineError> {
    // set a 100 
    // get a
    
    let mut output = Vec::new();
    for line in input.lines() {
        let command: Vec<_> = line.split_ascii_whitespace().collect();

        match command.get(0) {
            Some(operator) => {
                match *operator {
                    "set" => output.push(parse_set(&command)?),
                    "get" => output.push(parse_get(&command)?),
                    _ => return Err(EngineError::IdkWhatDisIs)
                }},
            None => return Err(EngineError::IdkWhatDisIs),
        }
    }
    Ok(output)
}

fn process(line: &String) -> Result<Value, EngineError> {
    let commands = parse(&line)?;
    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(&commands)?;
    Ok(result)
}

fn process_realtime(line: &String, evaluator: &mut Evaluator) -> Result<Value, EngineError> {
    let commands = parse(&line)?;
    let result = evaluator.evaluate(&commands)?;
    Ok(result)
}

fn main () {
    let args: Vec<String> = env::args()
                                .enumerate()
                                .filter(|&(index, _)| index != 0)
                                .map(|(_, elem)| elem)
                                .collect();
    if args.len() != 0 {
        for file in args{

                let input = fs::read_to_string(&file).expect("ouch!");
                match process(&input) {
                    Ok(value) => {
                        match value {
                            Value::Int(num) => println!("{}", num),
                            _ => (),
                        }
                    },
                    Err(_) => {eprintln!("What do you mean to say in {}?\n", file)}
                }
        }
    } else if args.len() == 0 {
        let mut input = String::new();
        let mut evaluator = Evaluator::new();
        let mut result = io::stdin().read_line(&mut input);
        while !input.contains("exit") {
            match result {
                Ok(_) => {
                    match process_realtime(&input, &mut evaluator) {
                        Ok(result) =>   match result {
                            Value::Int(val) => println!(">>>{}\n", val),
                            _ => print!(""),
                        },
                        Err(_) => eprintln!("That's not a good attitude i must say :(\n")
                    }
                }
                Err(_) => eprintln!("Okayy that flew over my head\n")
            }
            result = io::stdin().read_line(&mut input);
        }
    }
}

#[test]
fn test_1 () -> Result<(), EngineError> {
    let commands = vec![
        Command::SetVar("a".into(), Value::Int(100)), 
        Command::GetVar("a".into())
        ];

    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Int(100));

    Ok(())
}

#[test]
fn parse_test () -> Result<(), EngineError> {
    let input = "set x 30\nget x";

    let commands = parse(input)?;
    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(&commands)?;
    assert_eq!(result, Value::Int(30));

    Ok(())
}

#[test]
fn read_parse_test () -> Result<(), EngineError> {
    let input = fs::read_to_string("file.mad").expect("ouch!");

    let commands = parse(&input)?;
    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(&commands)?;
    assert_eq!(result, Value::Int(200));

    Ok(())
}