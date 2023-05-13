// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/5.0.1/nom/
// If you want to use it in your parser, you need to import it here. I've already imported a couple.
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, alphanumeric1, char, digit1, space0, space1},
    combinator::{map, opt},
    error::{Error, ErrorKind},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded},
    IResult,
};
  // Here are the different node types. You will use these to make your parser and your grammar.
  // You may add other nodes as you see fit, but these are expected by the runtime.
  #[derive(Debug, Clone)]
  pub enum Node {
    Program { children: Vec<Node> },
    Statement { children: Vec<Node> },
    FunctionReturn { children: Vec<Node> },
    FunctionDefine { children: Vec<Node> },
    FunctionArguments { children: Vec<Node> },
    FunctionStatements { children: Vec<Node> },
    Expression { children: Vec<Node> },
    MathExpression {name: String, children: Vec<Node> },
    FunctionCall { name: String, children: Vec<Node> },
    VariableDefine { children: Vec<Node> },
    Number { value: i32 },
    Bool { value: bool },
    Identifier { value: String },
    String { value: String },
    ComparisonExpression { name: String, children: Vec<Node> },
    IfStatement { children: Vec<Node> },
    ElseStatement { children: Vec<Node> },
    ElseIfStatement { children: Vec<Node> },
  }
  // Define production rules for an identifier
  pub fn identifier(input: &str) -> IResult<&str, Node> {
    let (input, result) = alphanumeric1(input)?;              // Consume at least 1 alphanumeric character. The ? automatically unwraps the result if it's okay and bails if it is an error.
    Ok((input, Node::Identifier{ value: result.to_string()})) // Return the now partially consumed input, as well as a node with the string on it.
  }
  // Define an integer number
  pub fn number(input: &str) -> IResult<&str, Node> {
    let (input, result) = digit1(input)?;                     // Consume at least 1 digit 0-9
    let number = result.parse::<i32>().unwrap();              // Parse the string result into a usize
    Ok((input, Node::Number{ value: number}))                 // Return the now partially consumed input with a number as well
  }
  pub fn boolean(input: &str) -> IResult<&str, Node> {
    let (input, result) = alt((tag("true"),tag("false")))(input)?;
    let bool_value = if result == "true" {true} else {false};
    Ok((input, Node::Bool{ value: bool_value}))
  }
  pub fn string(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("\"")(input)?;
    let (input, string) = many1(alt((alphanumeric1,tag(" "))))(input)?;
    let (input, _) = tag("\"")(input)?;
    Ok((input, Node::String{ value: string.join("")}))
  }
  pub fn function_call(input: &str) -> IResult<&str, Node> {
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, mut args) = many0(arguments)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Node::FunctionCall{name: name.to_string(), children: args}))   
  }
  pub fn parenthetical_expression(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, args) = l1(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    Ok((input, args))
  }
  pub fn l4(input: &str) -> IResult<&str, Node> {
    alt((function_call, number, identifier, parenthetical_expression))(input)
  }
  pub fn l3_infix(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?;
    let (input, op) = tag("^")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, args) = l4(input)?;
    Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
  }
  pub fn l3(input: &str) -> IResult<&str, Node> {
    let (input, mut head) = l4(input)?;
    let (input, tail) = many0(l3_infix)(input)?;
    for n in tail {
      match n {
        Node::MathExpression{name, mut children} => {
          let mut new_children = vec![head.clone()];
          new_children.append(&mut children);
          head = Node::MathExpression{name, children: new_children};
        }
        _ => () 
      };
    }
    Ok((input, head))
  }
  pub fn l2_infix(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?;
    let (input, op) = alt((tag("*"),tag("/")))(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, args) = l2(input)?;
    Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
  }
  pub fn l2(input: &str) -> IResult<&str, Node> {
    let (input, mut head) = l3(input)?;
    let (input, tail) = many0(l2_infix)(input)?;
    for n in tail {
      match n {
        Node::MathExpression{name, mut children} => {
          let mut new_children = vec![head.clone()];
          new_children.append(&mut children);
          head = Node::MathExpression{name, children: new_children};
        }
        _ => () 
      };
    }
    Ok((input, head))
  }
  pub fn l1_infix(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?;
    let (input, op) = alt((tag("+"),tag("-")))(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, args) = l2(input)?;
    Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
  }
  pub fn l1(input: &str) -> IResult<&str, Node> {
    let (input, mut head) = l2(input)?;
    let (input, tail) = many0(l1_infix)(input)?;
    for n in tail {
      match n {
        Node::MathExpression{name, mut children} => {
          let mut new_children = vec![head.clone()];
          new_children.append(&mut children);
          head = Node::MathExpression{name, children: new_children};
        }
        _ => () 
      };
    }
    Ok((input, head))
  }
  pub fn math_expression(input: &str) -> IResult<&str, Node> {
    l1(input)
  }
  pub fn expression(input: &str) -> IResult<&str, Node> {
    let (input, result) = alt((boolean, comparison, math_expression, function_call, number, string, identifier))(input)?;
    Ok((input, Node::Expression{ children: vec![result]}))   
  }
  pub fn statement(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?;
    let (input, result) = alt((variable_define, function_return, else_if_statement, else_statement, if_statement))(input)?;
    let (input, _) = tag(";")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = many0(tag("\n"))(input)?;
    Ok((input, Node::Statement{ children: vec![result]}))   
  }
  pub fn function_return(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("return ")(input)?;
    let (input, return_value) = alt((function_call, expression, identifier))(input)?;
    Ok((input, Node::FunctionReturn{ children: vec![return_value]}))
  }
  pub fn variable_define(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("let ")(input)?;
    let (input, variable) = identifier(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, expression) = expression(input)?;
    Ok((input, Node::VariableDefine{ children: vec![variable, expression]}))   
  }
  pub fn arguments(input: &str) -> IResult<&str, Node> {
    let (input, arg) = expression(input)?;
    let (input, mut others) = many0(other_arg)(input)?;
    let mut args = vec![arg];
    args.append(&mut others);
    Ok((input, Node::FunctionArguments{children: args}))
  }
  pub fn other_arg(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag(",")(input)?;
    expression(input)
  }
  pub fn function_definition(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("fn ")(input)?;
    let (input, function_name) = identifier(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, mut args) = many0(arguments)(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = many0(tag("\n"))(input)?;
    let (input, mut statements) = many1(statement)(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = many0(alt((tag("\n"),tag(" "))))(input)?;
    let mut children = vec![function_name];
    println!("args, {:?}", args);
    children.append(&mut args);
    children.append(&mut statements);
    Ok((input, Node::FunctionDefine{ children: children }))   
  }
  pub fn if_statement(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("if")(input)?;
    let (input, _) = space1(input)?;
    let (input, comparison) = comparison(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = space0(input)?;
    let (input, statements) = many0(delimited(space0, statement, space0))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("}")(input)?;

    let children = vec![
        comparison,
        Node::Statement {
            children: statements,
        },
    ];
    Ok((input, Node::IfStatement { children }))
}
pub fn else_statement(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("else")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = space0(input)?;
    let (input, statements) = many0(delimited(space0, statement, space0))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("}")(input)?;
    let children = vec![Node::Statement {
        children: statements,
    }];
    Ok((input, Node::ElseStatement { children }))
}
pub fn else_if_statement(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("else if")(input)?;
    let (input, _) = space1(input)?;
    let (input, comparison) = comparison(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = space0(input)?;
    let (input, statements) = many0(delimited(space0, statement, space0))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("}")(input)?;

    let children = vec![
        comparison,
        Node::Statement {
            children: statements,
        },
    ];
    Ok((input, Node::ElseIfStatement { children }))
}

// value = number | identifier | boolean;
pub fn value(input: &str) -> IResult<&str, Node> {
    alt((boolean, number, identifier))(input).map(|(i, node)| (i.trim_start(), node))
}


pub fn comparison(input: &str) -> IResult<&str, Node> {
    let (input, left) = value(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, operator) = alt((
        tag("=="), tag("!="), tag("<="), tag(">="), tag("<"), tag(">")
    ))(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, right) = value(input)?;
    let children = vec![left, right];
    let name = match operator {
        "==" => "==",
        "!=" => "!=",
        "<=" => "<=",
        ">=" => ">=",
        "<" => "<",
        ">" => ">",
        _ => return Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Tag)))
    };
    Ok((input, Node::ComparisonExpression { name: name.to_string(), children }))
}


  // Define a program. You will change this, this is just here for example.
  // You'll probably want to modify this by changing it to be that a program
  // is defined as at least one function definition, but maybe more. Start
  // by looking up the many1() combinator and that should get you started.
  pub fn program(input: &str) -> IResult<&str, Node> {
    let (input, result) = many1(alt((function_definition, statement, expression)))(input)?;  // Now that we've defined a number and an identifier, we can compose them using more combinators. Here we use the "alt" combinator to propose a choice.
    Ok((input, Node::Program{ children: result}))       // Whether the result is an identifier or a number, we attach that to the program
  }  



// pub fn comparison_expression(input: &str) -> IResult<&str, Node> {
//   let (input, left) = simple_expression(input)?;  // parse the left-hand side
//   let (input, _) = space0(input)?;  // consume any whitespace
//   let (input, operator) = alt((tag(">"), tag("<"), tag("=="), tag("!="), tag(">="), tag("<=")))(input)?;  // parse the operator
//   let (input, _) = space0(input)?;  // consume any whitespace
//   let (input, right) = simple_expression(input)?;  // parse the right-hand side

//   let node = Node::ComparisonExpression {
//       name: operator.to_string(),
//       children: vec![left, right],
//   };

//   Ok((input, node))
// }
