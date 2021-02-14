pub struct Vietnamese {
    pub name: String,
    pub age: u8,
}

pub struct American {
    pub name: String,
    pub age: u8,
}

pub trait Human {
    fn greet(&self) -> String;
    fn test_english(&self) -> bool {
        true
    }
}

pub trait Asian {
    fn stay_with_family(&self) -> String;
}

impl Human for Vietnamese {
    fn greet(&self) -> String {
        let greeting: String = String::from("xin chào");
        let name: &String = &self.name;
        let age: &u8 = &self.age;
        let info: String = name.to_string() + " người có tuổi " + &age.to_string();
        let mut say_eng: String = String::from("nói được tiếng Anh");
        if !self.test_english() {
            say_eng = String::from("không nói được tiếng Anh");
        };
        info + " nói " + &greeting + ". Và " + name + " " + &say_eng + "."
    }

    fn test_english(&self) -> bool {
        false
    }
}

impl Asian for Vietnamese {
    fn stay_with_family(&self) -> String {
        format!("{} sống chung với gia đình", self.name)
    }
}

impl Human for American {
    fn greet(&self) -> String {
        let greeting: String = String::from("hello");
        let name: &String = &self.name;
        let age: &u8 = &self.age;
        let info: String = name.to_string() + " who\'s age " + &age.to_string();
        let mut say_eng: String = String::from("can speak English");
        if !self.test_english() {
            say_eng = String::from("cannot speaks English");
        };
        format!("{} says {}. And {} {}.", info, greeting, name, say_eng)
    }
}
