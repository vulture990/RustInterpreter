extern crate asalang;
extern crate nom;

use asalang::{program, Node, Value, start_interpreter};
use nom::IResult;

macro_rules! test {
  ($func:ident, $test:tt, $expected:expr) => (
    #[test]
    fn $func() -> Result<(),String> {
      match program($test) {
        Ok((input, p)) => {
          assert_eq!(input, "");
          assert_eq!(start_interpreter(&p), $expected);
          Ok(())
        },
        Err(e) => Err(format!("{:?}",e)),
      }
    }
  )
}

test!(numeric, r#"123"#, Ok(Value::Number(123)));
test!(identifier, r#"x"#, Err("Undefined variable"));
test!(string, r#""hello world""#, Ok(Value::String("hello world".to_string())));
test!(bool_true, r#"true"#, Ok(Value::Bool(true)));
test!(bool_false, r#"false"#, Ok(Value::Bool(false)));
test!(function_call, r#"foo()"#, Err("Undefined function"));
test!(function_call_one_arg, r#"foo(a)"#, Err("Undefined function"));
test!(function_call_more_args, r#"foo(a,b,c)"#, Err("Undefined function"));
test!(variable_define, r#"let x = 123;"#, Ok(Value::Number(123)));
test!(variable_init, r#"let x = 1;"#, Ok(Value::Number(1)));
test!(variable_bool, r#"let bool = true;"#, Ok(Value::Bool(true)));
test!(variable_string, r#"let string = "Hello World";"#, Ok(Value::String("Hello World".to_string())));
test!(variable_init_no_space, r#"let x=1;"#, Ok(Value::Number(1)));
test!(math, r#"1 + 1"#, Ok(Value::Number(2)));
test!(math_no_space, r#"1+1"#, Ok(Value::Number(2)));
test!(math_subtraction, r#"1 - 1"#, Ok(Value::Number(0)));
test!(math_multiply, r#"2 * 4"#, Ok(Value::Number(8)));
test!(math_divide, r#"6 / 2"#, Ok(Value::Number(3)));
test!(math_exponent, r#"2 ^ 4"#, Ok(Value::Number(16)));
test!(math_more_terms, r#"10 + 2*6"#, Ok(Value::Number(22)));
test!(math_more_terms_paren, r#"((10+2)*6)/4"#, Ok(Value::Number(18)));
test!(assign_math, r#"let x = 1 + 1;"#, Ok(Value::Number(2)));
test!(assign_function, r#"let x = foo();"#, Err("Undefined function"));
test!(assign_function_arguments, r#"let x = foo(a,b,c);"#, Err("Undefined function"));
test!(define_function, r#"fn main(){return foo();} fn foo(){return 5;}"#, Ok(Value::Number(5)));
test!(define_function_args, r#"fn main(){return foo(1,2,3);} fn foo(a,b,c){return a+b+c;}"#, Ok(Value::Number(6)));
test!(define_function_more_statement, r#"fn main() {
  return foo();
}
fn foo(){
  let x = 5;
  return x;
}"#, Ok(Value::Number(5)));
test!(define_full_program, r#"fn foo(a,b,c) {
  let x = a + 1;
  let y = bar(c - b);
  return x * y;
}

fn bar(a) {
  return a * 3;
}

fn main() {
  return foo(1,2,3);  
}"#, Ok(Value::Number(6)));
//test!(if_less_than, r#"if 2 < 3 { return "2 is less than 3"; } else { return "2 is not less than 3"; }"#, Ok(Value::String("2 is less than 3".to_string())));
//-------Comparison Tests-------
test!(comparison_main, r#"fn main() { return 2 < 3; }"#, Ok(Value::Bool(true)));
test!(comparison_less_than_equal, r#"2 <= 3"#, Ok(Value::Bool(true)));
test!(comparison_greater_than, r#"2 > 3"#, Ok(Value::Bool(false)));
test!(comparison_greater_than_equal, r#"2 >= 3"#, Ok(Value::Bool(false)));
//let result = x + y * z > x * y - z == true;
test!(comparison_main_set_variable, r#"fn main() { let x = 10; let y = 5; let z = 3; return x + y + z;}"#, Ok(Value::Number(18)));
test!(invalidComparison, r#"1 > true"#, Err("Invalid comparison operands"));
test!(invalidComparison2, r#"x + y * z > x * y - z == false"#, Ok(Value::Bool(true)));
// test!(invalidComparison2, r#"5 - false"#, Err("Invalid comparison operands"));
//problems start here
//test!(comparison_main_set_variable2, r#"fn main() { let x = 10; let y = 5; let z = 3; let result = x + y * z > x * y - z == true;}"#, Ok(Value::Bool(true)));