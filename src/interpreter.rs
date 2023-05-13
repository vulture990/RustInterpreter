use crate::parser::Node;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Number(i32),
  Bool(bool),
}


struct Runtime {
  functions: HashMap<String, Vec<Node>>,
  stack: Vec<HashMap<String, Value>>,
}

impl Runtime {

  pub fn new() -> Runtime {
    Runtime {
      functions: HashMap::new(),
      stack: Vec::new(),
    }
  }

  // Define the `run` method of the `Runtime` struct.
  pub fn run(&mut self, node: &Node) -> Result<Value, &'static str> {
    // Match the type of the input `Node`.
    match node {
        // If the `Node` is a `Program`, evaluate each of its children in sequence.
        Node::Program { children } => {
            for n in children {
                match n {
                    // If the child node is a `FunctionDefine`, add it to the list of functions.
                    Node::FunctionDefine { .. } => {
                        self.run(n);
                    },
                    // If the child node is an `Expression`, add it as the body of a new `main` function.
                    Node::Expression { .. } => {
                        self.functions.insert("main".to_string(), vec![Node::FunctionReturn { children: vec![n.clone()] }]);
                    },
                    // If the child node is a `Statement`, add it as the body of a new `main` function.
                    Node::Statement { .. } => {
                        self.functions.insert("main".to_string(), vec![n.clone()]);
                    }
                    // Ignore any other type of child node.
                    _ => (),
                }
            }
            // Return `Value::Bool(true)` wrapped in a `Result`.
            Ok(Value::Bool(true))
        },
        // If the `Node` is a `MathExpression`, evaluate it.
        Node::MathExpression { name, children } => {
            // Evaluate the left and right children of the `MathExpression`.
            match (self.run(&children[0]), self.run(&children[1])) {
                // If both children are `Number` values, extract their values and evaluate the expression.
                (Ok(Value::Number(lhs)), Ok(Value::Number(rhs))) => {
                    match name.as_ref() {
                        // If the operator is `+`, add the values.
                        "+" => Ok(Value::Number(lhs + rhs)),
                        // If the operator is `-`, subtract the values.
                        "-" => Ok(Value::Number(lhs - rhs)),
                        // If the operator is `*`, multiply the values.
                        "*" => Ok(Value::Number(lhs * rhs)),
                        // If the operator is `/`, divide the values.
                        "/" => Ok(Value::Number(lhs / rhs)),
                        // If the operator is `^`, raise the left value to the power of the right value.
                        "^" => {
                            let mut result = 1;
                            for i in 0..rhs {
                                result = result * lhs;
                            }
                            Ok(Value::Number(result))
                        },
                        // If the operator is not recognized, return an error message.
                        _ => Err("Undefined operator"),
                    }
                }
                // If either child is not a `Number` value, return an error message.
                _ => Err("Cannot do math on String or Bool"),
            }
        },
        // If the `Node` is a `FunctionCall`, evaluate it.
        Node::FunctionCall { name, children } => {
            // Extract the input arguments.
            let in_args = if children.len() > 0 {
                match &children[0] {
                    Node::FunctionArguments { children } => {
                        children
                    },
                    _ => children,
                }
            } else {
                children
            };
            // Create a new frame for local variables.
            let mut new_frame = HashMap::new();
            // Initialize the result to an error message.
            let mut result: Result<Value, &'static str> = Err("Undefined function");
            // Save a raw pointer to the `Runtime` instance for use in the nested closure.
            let rt = self as *mut Runtime;
            // Find the named function and evaluate its body.
            match self.functions.get(name) {
                Some(statements) => {
                    {
                        // If the function has input arguments, bind their values to the corresponding parameters.
                        match statements[0].clone() {
                            Node::FunctionArguments { children } => {
                                for (ix, arg) in children.iter().enumerate() {
                                    // Use unsafe Rust code to call `run` on the input argument and handle any errors.
                                    unsafe {
                                        let result = (*rt).run(&in_args[ix])?;
                                        match arg {
                                            Node::Expression { children } => {
                                                match &children[0] {
                                                    Node::Identifier { value } => {
                                                        new_frame.insert(value.clone(), result);
                                                    },
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                    // Push the new frame onto the stack.
                    self.stack.push(new_frame);
                    // Evaluate each statement in the function body.
                    for n in statements.clone() {
                        result = self.run(&n);
                    }
                    // Pop the frame off the stack.
                    self.stack.pop();
                },
                None => (),
            };
            // Return the result of evaluating the function.
            result
        },
        // If the `Node` is a `FunctionDefine`, add it to the list of functions.
        Node::FunctionDefine { children } => {
            let (head, tail) = children.split_at(1);
            match &head[0] {
                Node::Identifier { value } => {
                    self.functions.insert(value.to_string(), tail.to_vec());
                },
                _ => (),
            }
            Ok(Value::Bool(true))
        },
        // If the `Node` is a `FunctionReturn`, evaluate its child node.
        Node::FunctionReturn { children } => {
            self.run(&children[0])
        },
        // If the `Node` is an `Identifier`, look up its value in the current frame.
        Node::Identifier { value } => {
            let last = self.stack.len() - 1;
            match self.stack[last].get(value) {
                Some(id_value) => Ok(id_value.clone()),
                None => Err("Undefined variable"),
            }
        },
        // If the `Node` is a `Statement`, evaluate its child node.
        Node::Statement { children } => {
            match children[0] {
                Node::VariableDefine { .. } |
                Node::FunctionReturn { .. } => {
                    self.run(&children[0])
                },
                _ => Err("Unknown Statement"),
            }
        },
        // If the `Node` is a `VariableDefine`, evaluate its expression and bind the result to a new variable.
        Node::VariableDefine { children } => {
            // Extract the variable name.
            let name: String = match &children[0] {
                Node::Identifier { value } => value.clone(),
                _ => "".to_string(),
            };
            // Evaluate the expression.
            let value = self.run(&children[1])?;
            // Add the variable to the current frame.
            let last = self.stack.len() - 1;
            self.stack[last].insert(name, value.clone());
            // Return the value.
            Ok(value)
        }
        // If the `Node` is an `Expression`, evaluate its child node.
        Node::Expression { children } => {
            match children[0] {
                Node::ComparisonExpression { .. } => {
                    self.run(&children[0])
                },
                Node::MathExpression { .. } |
                Node::Number { .. } |
                Node::FunctionCall { .. } |
                Node::String { .. } |
                Node::Bool { .. } |
                Node::Identifier { .. } => {
                    self.run(&children[0])
                },
                _ => Err("Unknown Expression"),
            }
        },

        // If the `Node` is a `ComparisonExpression`, evaluate it.
        Node::ComparisonExpression { name, children } => {
            let left_value = self.run(&children[0])?;
            let right_value = self.run(&children[1])?;
        
            match (left_value, right_value) {
                (Value::Number(left), Value::Number(right)) => {
                    match name.as_str() {
                        "==" => Ok(Value::Bool(left == right)),
                        "!=" => Ok(Value::Bool(left != right)),
                        "<=" => Ok(Value::Bool(left <= right)),
                        ">=" => Ok(Value::Bool(left >= right)),
                        "<" => Ok(Value::Bool(left < right)),
                        ">" => Ok(Value::Bool(left > right)),
                        _ => Err("Undefined operator"),
                    }
                },
                (Value::Bool(left), Value::Bool(right)) => {
                    match name.as_str() {
                        "==" => Ok(Value::Bool(left == right)),
                        "!=" => Ok(Value::Bool(left != right)),
                        _ => Err("Invalid comparison operation for booleans"),
                    }
                },
                _ => Err("Invalid comparison operands"),
            }
        },
        
        // If the `Node` is a `Number`, wrap its value in a `Value::Number` and return it.
        Node::Number { value } => {
            Ok(Value::Number(*value))
        }
        // If the `Node` is a `String`, wrap its value in a `Value::String` and return it.
        Node::String { value } => {
            Ok(Value::String(value.clone()))
        }
        // If the `Node` is a `Bool`, wrap its value in a `Value::Bool` and return it.
        Node::Bool { value } => {
            Ok(Value::Bool(*value))
        }
        // If the `Node` is of an unhandled type, return an error message.
        _ => {
            Err("Unhandled Node")
        },
    }
  }
}

pub fn start_interpreter(node: &Node) -> Result<Value, &'static str> {
  let mut runtime = Runtime::new();
  runtime.run(node);
  let start_main = Node::FunctionCall{name: "main".to_string(), children: vec![]};
  runtime.run(&start_main)
}