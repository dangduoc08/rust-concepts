#[derive(Debug)]
pub enum Status {
  Succeeded,
  Failed,
  Pending,
}

#[derive(Debug)]
pub enum Component {
  GraphicCard(Status),
  CPU(Status),
  Ram(Status),
}

pub fn run() {
  let a_computer_component = Some(Component::CPU(Status::Pending));
  println!("PATTERN 1: match Arms");

  let res: &str = match a_computer_component {
    Some(Component::GraphicCard(Status::Succeeded)) => "GraphicCard succeeded",
    Some(Component::GraphicCard(Status::Failed)) => "GraphicCard failed",
    Some(Component::GraphicCard(Status::Pending)) => "GraphicCard pending",
    Some(Component::CPU(Status::Succeeded)) => "CPU succeeded",
    Some(Component::CPU(Status::Failed)) => "CPU failed",
    Some(Component::CPU(Status::Pending)) => "CPU pending",
    Some(Component::Ram(Status::Succeeded)) => "Ram succeeded",
    Some(Component::Ram(Status::Failed)) => "Ram failed",
    Some(Component::Ram(Status::Pending)) => "Ram pending",
    None => "None",
    // _ => "Not import computer component parts",
  };
  println!("Final match: {:?}", res);
  println!("");

  println!("PATTERN 2: Conditional if let Expressions");
  let favorite_color: Option<&str> = None;
  let is_tuesday = false;
  let age: Result<u8, &str> = Ok(26);

  if let Some(color) = favorite_color {
    println!("Favorite color is {}", color)
  } else if is_tuesday {
    println!("Tuesday is green day!");
  } else if let Ok(age) = age {
    if age > 30 {
      println!("Favorite color is purple");
    } else {
      println!("Favorite color is orange");
    }
  } else {
    println!("Blue as default color");
  }
  println!("");

  println!("PATTERN 3: while let Conditional Loops");
  let mut stack: Vec<isize> = Vec::new();
  stack.push(10);
  stack.push(20);
  stack.push(30);
  stack.push(40);
  stack.push(50);

  while let Some(test) = stack.pop() {
    println!("{:?}", test)
  }
  println!("");

  println!("PATTERN 4: for Loops");
  let v = vec!['a', 'b', 'c'];

  for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
  }
  println!("");

  println!("PATTERN 5: let Statements");
  let (x, y, z, _, ..) = (10, 9, "string", "hihi", "hah"); // .. same as _
  println!("x {}, y {}, z {}", x, y, z);
  println!("");

  println!("PATTERN 6: Function Parameters");
  fn print_coordinates(&(x, y): &(u8, u8)) {
    println!("Current location: ({}, {})", x, y);
  }

  let point: (u8, u8) = (3, 5);
  print_coordinates(&point);
}
