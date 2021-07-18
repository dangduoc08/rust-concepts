// Abstraction
// if struct act like a live object
// and yell live a live object
// it's must be a live object
// treat trail like interface
pub trait LiveObject {
  fn act(&self);

  // Polymorphism
  // by default all struct implemt this trait will have default yell method
  // it's also can overide this method
  // so it's maybe a live object or it's a Person or an Animal
  fn yell(&self) {
    println!("This is default yell");
  }
}

pub struct Farm {
  // Trail object
  pub objects: Vec<Box<dyn LiveObject>>,
}

impl Farm {
  pub fn make_object_act(&self) {
    for object in self.objects.iter() {
      object.act();
    }
  }

  pub fn make_object_yell(&self) {
    for object in self.objects.iter() {
      object.yell();
    }
  }
}

#[derive(Debug)]
pub enum Nationality {
  America,
  Vietnam,
}

#[derive(Debug)]
pub enum Gender {
  Male,
  Female,
  Gay,
  Bisexual,
}

// Encapsulation
// pub for public properties / methods
// non-pub for private properties / methods
#[derive(Debug)]
pub struct Person<'a, 'b> {
  pub name: &'a str,
  pub age: i8,
  pub gender: Gender,
  pub nationality: Nationality,
  account_balance: isize,
  address: &'b str,
}

impl<'a, 'b> Person<'a, 'b> {
  pub fn new(
    name: &'a str,
    age: i8,
    gender: Gender,
    account_balance: isize,
    nationality: Nationality,
    address: &'b str,
  ) -> Person<'a, 'b> {
    Person {
      name,
      age,
      gender,
      account_balance,
      nationality,
      address,
    }
  }
}

impl<'a, 'b> LiveObject for Person<'a, 'b> {
  fn act(&self) {
    println!("{} is acting to {}", self.name, self.address);
  }

  fn yell(&self) {
    let greet = match self.nationality {
      Nationality::America => "hello",
      Nationality::Vietnam => "xin chào",
    };

    println!("{} yells \"{}\"", self.name, greet);
  }
}

#[derive(Debug)]
pub enum AnimalKind {
  Poultry,
  Cattle,
}

#[derive(Debug)]
pub struct Animal<'a> {
  pub kind: AnimalKind,
  pub name: &'a str,
}

impl<'a> LiveObject for Animal<'a> {
  fn act(&self) {
    let action = match self.kind {
      AnimalKind::Poultry => "flying",
      AnimalKind::Cattle => "running",
    };

    println!("{:?} is {}", self.kind, action);
  }

  // Inheritance
  // if yell was not overridden
  // it's inherit from LiveObject trails's default yell method
  fn yell(&self) {
    let sound = match self.kind {
      AnimalKind::Poultry => "cococo",
      AnimalKind::Cattle => "grrrr",
    };

    println!("{:?} yells {}", self.kind, sound);
  }
}

pub fn run() {
  let john_doe = Person::new(
    "John Doe",
    26,
    Gender::Male,
    26000000,
    Nationality::America,
    "601 13th Street NW Suite 900 South",
  );

  let cow = Animal {
    name: "Daisy",
    kind: AnimalKind::Cattle,
  };

  let da_lat_farm = Farm {
    objects: vec![Box::new(john_doe), Box::new(cow)],
  };

  da_lat_farm.make_object_act();
  da_lat_farm.make_object_yell();
}
