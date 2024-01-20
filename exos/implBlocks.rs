pub struct Animal {
    race: String,
    name: String,
    age: i32,
}

impl Animal {
    fn get_name(&self) -> &String{
        &self.name
    }
    fn get_race(&self) -> &String{
        &self.race
    }
}

fn main() {
    let cat = Animal {
        name: "Fluppy".to_string(),
        race: "Cat".to_string(),
        age: 2,
    };

    println!(
        "This animal is a {} and his name is {}",
        cat.get_race(),
        cat.get_name()
    );
}
