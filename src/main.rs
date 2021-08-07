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
enum Error {
    MissingVariable(String),

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

    fn evaluate(&mut self, commands: &[Command]) -> Result<Value, Error> {
        let mut output = Ok(Value::Nothing);
        for command in commands {
            match command {
                Command::SetVar(name, value) => {
                    self.vars.insert(name.into(), value.clone());
                }
                Command::GetVar(name) => {
                    match self.vars.get(name){
                        Some(value) =>  output = Ok(value.clone()),
                        None => return Err(Error::MissingVariable(name.into())),
                    }
                }
            }
        }
        output
    }
}
fn main () {
    
}

#[test]
fn test_1 () -> Result<(), Error> {
    let commands = vec![
        Command::SetVar("a".into(), Value::Int(100)), 
        Command::GetVar("a".into())
        ];

    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Int(100));

    Ok(())
}