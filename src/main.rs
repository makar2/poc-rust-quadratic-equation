use std::io;
use std::io::Write; // <--- bring flush() into scope

fn main() {
  let a: f32 = get_input("a = "); // e.g. 1
  let b: f32 = get_input("b = "); // e.g. 4
  let c: f32 = get_input("c = "); // e.g. -5

  print_equation(a, b, c);

  let discriminant = get_discriminant(a, b, c);

  if discriminant < 0.0 {
    println!("The Discriminant is less than 0, meaning the equation has no answer :(");
  }
  else if discriminant == 0.0 {
    let x = - (b / (2.0 * a));
    println!("Answer: x = {}", x);
  }
  else {
    let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
  
    println!("Answer 1: x = {}", x1);
    println!("Answer 2: x = {}", x2);
  }

  // println!("Check 1: 0 = {}", a * x1.powi(2) + b * x1 + c);
  // println!("Check 2: 0 = {}", a * x2.powi(2) + b * x2 + c);
}

fn get_input (question: &str) -> f32 {
  let mut provided_string = String::new();
  print!("{}", question);
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut provided_string);
  let output: f32 = provided_string.trim().parse::<f32>().unwrap();
  return output;
}

fn print_equation(a: f32, b: f32, c: f32) {
  let a_final = decorate(a, "x²".to_string(), 1);
  let b_final = decorate(b, "x".to_string(), 2);
  let c_final = decorate(c, "".to_string(), 3);
  println!("{}{}{} = 0", a_final, b_final, c_final);
}

fn decorate (number: f32, variable: String, order: i32) -> String {
  let prefix =
    if number >= 0.0 && order != 1 {
      "+".to_string()
    }
    else {
      "".to_string()
    };

  let number_final =
    if number == 1.0 && order != 3 {
      "".to_string()
    }
    else {
      number.to_string()
    };

  return prefix + &number_final + &variable;
}

fn get_discriminant (a: f32, b: f32, c: f32) -> f32 {
  let discriminant = b.powi(2) - (4.0 * a * c);
  println!("D = {}", discriminant);
  return discriminant;
}