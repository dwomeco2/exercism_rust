#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for i in inputs {
        if let CalculatorInput::Value(num) = *i {
            stack.push(num)
        } else {
            if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
                match *i {
                    CalculatorInput::Add => {
                        stack.push(a + b);
                    }
                    CalculatorInput::Subtract => {
                        stack.push(b - a);
                    }
                    CalculatorInput::Multiply => {
                        stack.push(b * a);
                    }
                    CalculatorInput::Divide => {
                        stack.push(b / a);
                    }
                    _ => panic!("what is this!!"),
                }
            } else {
                return None;
            }
        }
    }
    if stack.len() > 1 {
        return None;
    }
    stack.pop()
}
