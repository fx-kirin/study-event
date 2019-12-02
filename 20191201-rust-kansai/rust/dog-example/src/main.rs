struct Dog {
    name: String,
    age: u64,
}

impl Dog {
    pub fn description(&self) -> String {
        format!("{}, a {} years old dog", self.name, self.age)
    }

    pub fn speak_with_description(&self, sound: String){
        println!("\"{}\" said {}.", sound, self.description());
    }
}


fn main() {
    let dog = Dog{ name:"kevin".to_string(), age:3 };
    dog.speak_with_description("Bow!".to_string());
}
