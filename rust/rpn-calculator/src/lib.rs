#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stk = vec![];
    for input in inputs {
        match input {
            CalculatorInput::Add => {
                match stk.pop() {
                    Some(a) => match stk.pop() {
                        Some(b) => stk.push(b + a),
                        None => return None,
                    },
                    None => return None,
                }
            } 
            CalculatorInput::Subtract => {
                match stk.pop() {
                    Some(a) => match stk.pop() {
                        Some(b) => stk.push(b - a),
                        None => return None,
                    },
                    None => return None,
                }          
            }
            CalculatorInput::Multiply => {
                match stk.pop() {
                    Some(a) => match stk.pop() {
                        Some(b) => stk.push(b * a),
                        None => return None,
                    },
                    None => return None,
                }           
            }
            CalculatorInput::Divide => {
                match stk.pop() {
                    Some(a) => match stk.pop() {
                        Some(b) => stk.push(b / a),
                        None => return None,
                    },
                    None => return None,
                }         
            }
            CalculatorInput::Value(val) => stk.push(*val)
        }
    }
    if stk.len() == 1 {
        Some(stk.pop().unwrap())
    } else {
        None
    }
}
