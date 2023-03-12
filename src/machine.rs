use crate::register::Register;
use crate::stack::Stack;

use std::{
    default::Default,
    error::Error,
    io::{self, Write},
};

type Tokens = Vec<String>;

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Xor,
    And,
    Or,
    Shr,
    Shl,
}

#[derive(Debug, Default)]
pub struct Machine {
    r1: Register,
    r2: Register,
    stack: Stack,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            r1: Register::new(),
            r2: Register::new(),
            stack: Stack::new(),
        }
    }

    fn do_operation(
        &mut self,
        operator: Operation,
        tokens: &Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        match tokens.len() {
            3 => {
                let num1: i64;
                let num2: i64;

                num1 = match tokens[1].as_str() {
                    "r1" => self.r1.get(),
                    "r2" => self.r2.get(),
                    &_ => tokens[1].parse()?,
                };

                num2 = match tokens[2].as_str() {
                    "r1" => self.r1.get(),
                    "r2" => self.r2.get(),
                    &_ => tokens[2].parse()?,
                };

                match operator {
                    Operation::Add => self.stack.push(num1 + num2),
                    Operation::Sub => self.stack.push(num1 - num2),
                    Operation::Mul => self.stack.push(num1 * num2),
                    Operation::Div => {
                        if num2 == 0 {
                            return Err("div cannot divide by zero".into());
                        }
                        self.stack.push(num1 / num2);
                    }
                    Operation::Xor => self.stack.push(num1 ^ num2),
                    Operation::Or => self.stack.push(num1 | num2),
                    Operation::And => self.stack.push(num1 & num2),
                    Operation::Shr => self.stack.push(num1 >> num2),
                    Operation::Shl => self.stack.push(num1 << num2),
                }

                if let Some(peeked) = self.stack.peek() {
                    println!("{peeked}");
                }

                Ok(())
            }
            _ => return Err("mul must have two operands".into()),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut input = String::new();
        loop {
            print!("> ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut input)?;

            let tokens: Tokens = input.split_whitespace().map(str::to_string).collect();
            if let Err(e) = self.parse(tokens) {
                println!("[ERR] {}", e);
            }

            input.clear();
        }
    }

    fn parse(&mut self, tokens: Vec<String>) -> Result<(), Box<dyn Error>> {
        if tokens.is_empty() {
            return Err("Expected tokens to contain at least one item".into());
        }

        match tokens[0].as_str() {
            "exit" => {
                return Ok(());
            }
            "push" => match tokens.len() {
                2 => {
                    let num: i64;
                    match tokens[1].as_str() {
                        "r1" => num = self.r1.get(),
                        "r2" => num = self.r2.get(),
                        &_ => num = tokens[1].parse()?,
                    }

                    self.stack.push(num);
                    if let Some(peeked) = self.stack.peek() {
                        println!("{peeked}");
                    }
                }
                _ => return Err(format!("Malformed operation {:?}", tokens).into()),
            },
            "pop" => {
                if let Some(popped) = self.stack.pop() {
                    println!("{popped}");
                }
            }
            "peek" => {
                if let Some(peeked) = self.stack.peek() {
                    println!("{peeked}");
                }
            }
            "mov" => match tokens.len() {
                3 => {
                    let num: i64;
                    match tokens[2].as_str() {
                        "r1" => num = self.r1.get(),
                        "r2" => num = self.r2.get(),
                        "pop" => {
                            num = match self.stack.pop() {
                                Some(n) => n,
                                None => return Err("Stack is empty".into()),
                            }
                        }
                        "peek" => {
                            num = match self.stack.peek() {
                                Some(n) => *n,
                                None => return Err("Stack is empty".into()),
                            }
                        }
                        &_ => num = tokens[2].parse()?,
                    }

                    match tokens[1].as_str() {
                        "r1" => {
                            self.r1.set(num);
                            println!("{}", self.r1.get());
                        }
                        "r2" => {
                            self.r2.set(num);
                            println!("{}", self.r1.get());
                        }
                        &_ => return Err(format!("Unknown register {}", tokens[1]).into()),
                    }
                }
                _ => return Err(format!("Malformed operation {:?}", tokens).into()),
            },
            "r1" => println!("{}", self.r1.get()),
            "r2" => println!("{}", self.r2.get()),
            "add" => return self.do_operation(Operation::Add, &tokens),
            "sub" => return self.do_operation(Operation::Sub, &tokens),
            "mul" => return self.do_operation(Operation::Mul, &tokens),
            "div" => return self.do_operation(Operation::Div, &tokens),
            "xor" => return self.do_operation(Operation::Xor, &tokens),
            "or" => return self.do_operation(Operation::Or, &tokens),
            "and" => return self.do_operation(Operation::And, &tokens),
            "shr" => return self.do_operation(Operation::Shr, &tokens),
            "shl" => return self.do_operation(Operation::Shl, &tokens),
            &_ => {}
        }

        Ok(())
    }
}
