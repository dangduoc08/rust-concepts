pub struct Person<'a> {
  pub name: &'a str,
  pub age: usize,
}

pub fn run() {
  let x = 5;
  match x {
    4 | 5 => println!("four or five"),
    3 => println!("three"),
    _ => println!("anything"),
  };

  let h = 'h';
  match h {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
  };

  let nam = Person {
    name: "Nam",
    age: 26,
  };

  // Destructuring
  let Person { name, age } = nam;
  // or
  let Person {
    name: ten,
    age: tuoi,
  } = nam;

  println!("name {} or ten {}", name, ten);
  println!("age {} or tuoi {}", age, tuoi);

  match nam {
    Person { name, age: 30 } => println!("name {}, age {}", name, age), // name = any, age = 30
    Person { name: "Nam", age } => println!("name {}, age {}", name, age), // name = "Nam", age = any
    Person { name, age } => println!("name {}, age {}", name, age),        // name = any, age = any
  };

  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }

  let msg = Message::ChangeColor(0, 160, 255);

  match msg {
    Message::Quit => {
      println!("The Quit variant has no data to destructure.")
    }
    Message::Move { x, y } => {
      println!("Move in the x direction {} and in the y direction {}", x, y);
    }
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
      println!("Change the color to red {}, green {}, and blue {}", r, g, b)
    }
  };

  let mut setting_value = Some(5);
  let new_setting_value = Some(10);

  match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
      println!("Can't overwrite an existing customized value");
    }
    _ => {
      setting_value = new_setting_value;
    }
  }

  println!("setting is {:?}", setting_value);

  struct Point {
    x: i32,
    y: i32,
    z: i32,
  }

  let origin = Point { x: 0, y: 0, z: 0 };

  match origin {
    Point { x, .. } => println!("x is {}", x),
  }

  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    (first, .., last) => {
      println!("Some numbers: {}, {}", first, last);
    }
  }

  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {}", n),
    _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {}", x, y);

  enum Test {
    Hello { id: i32 },
  }

  let msg = Test::Hello { id: 5 };

  match msg {
    Test::Hello {
      id: id_variable @ 3..=7,
    } => println!("Found an id in range: {}", id_variable),
    Test::Hello { id: 10..=12 } => {
      println!("Found an id in another range")
    }
    Test::Hello { id } => println!("Found some other id: {}", id),
  }
}
