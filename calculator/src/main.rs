use std::env::{args, Args};

fn main() {
  println!("Hello, world! RUST¡");

  let mut args: Args= args();
  let first = args.nth(1).unwrap();
  
  //println!("{:?}",first);

  let operator = args.nth(0).unwrap().chars().next().unwrap();
  let second = args.nth(0).unwrap();
    
  let first_number = first.parse::<f32>().unwrap();
  let second_number: f32 = second.parse().unwrap();
  println!("{:?}{} {}",first, operator, second);
  println!("{}{}{}", first_number,operator, second_number);

  //operator= operator.chars().next().unwrap();

  let result = operate(operator, first_number, second_number);
  println!("{:?}",result);

  println!("{:?}", output(first_number,operator,second_number,result));
}

/*fn operate(operator:char, first_number: f32, second_number:f32) -> f32{
  if operator == '+'{
    first_number + second_number
  } else if operator == '-'{
    return first_number- second_number
  }else if operator == '/'{
    first_number/second_number
  }else if operator == '*'{
    return first_number*second_number;
  }else{
     0.0
  }
}*/

fn operate(operator:char, first_number: f32, second_number:f32) -> f32{
  match operator{
    '+' => first_number+second_number,
    '-' => first_number-second_number,
    '*' | 'X' | 'x' => first_number*second_number,
    '/' => first_number/second_number,
    _ => panic!("Invalid operator USED.")
  }
}

fn output(first_number:f32, operator:char, second_number:f32, result:f32) -> String {
  format!("{}{}{}={}", first_number, operator,second_number,result)
}