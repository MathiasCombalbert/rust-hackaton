struct School {
    school_name: String,
    location: String,
    address: String,
    is_rated: bool,
    rate: i32,
}

fn get_school_name(school: School) -> String {
    return school.school_name;
}

fn main() {
    let school = School {
        school_name: "Epitech".to_string(),
        location: "Toulouse".to_string(),
        address: "40 Boulevard de la marquette".to_string(),
        is_rated: true,
        rate: 5,
    };

    println!("{}", get_school_name(school));
}
