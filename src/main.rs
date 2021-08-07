use std::collections::HashMap; 

enum Command {
    setVar(String, Value),
    getVar(String)
}

#[derive(Clone)]
enum Value {
    Int(i64)
}

struct Evaluator {
    vars: HashMap<String, Value>,
}

impl Evaluator {


fn evaluate(&mut self, commands: &[Command]) -> Result<Value, Box<dyn std::error::Error>> {
    for command in commands {
        match command {
            Command::setVar(name, value) => {
                self.vars.insert(k: name.into(), v: value.clone())
            }
        }
    }
}

}
fn main () {
    
}

#[test]
fn test_1 () {
    let commands = vec![
        Command::setVar("a".into(), Value::Int(100)), 
        Command::getVar("a".into())
        ];

    let result = evaluate(&commands);
}