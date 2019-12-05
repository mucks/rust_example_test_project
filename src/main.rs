use std::fmt;

use chrono::NaiveDateTime;
use std::str::FromStr;

fn main() {
    let name = "Max";
    let mut s_result = MyStruct::new(name, 17.5544554);
    match s_result {
        Ok(s) => println!("{}", s),
        Err(err) => println!("{}", err),
    }

    if let Some(name) = can_happen("hello") {}
}

fn can_happen(name: &str) -> Option<String> {
    if name == "hello" {
        Some("".into())
    } else {
        None
    }
}

#[derive(Debug)]
pub struct MyStruct {
    name: String,
    age: f32,
    date: NaiveDateTime,
}

impl MyStruct {
    pub fn new(name: &str, age: f32) -> Result<Self, chrono::ParseError> {
        Ok(Self {
            name: name.into(),
            age,
            date: NaiveDateTime::parse_from_str(
                "2019-12-05T18:37:33.796Z",
                "%Y-%m-%dT%H:%M:%S%.fZ",
            )?,
        })
    }

    pub fn format_name(&mut self) {
        self.name = format!("My name is {}", self.name);
    }
}

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "My name is {} and I am {:.2} old", self.name, self.age)
    }
}
