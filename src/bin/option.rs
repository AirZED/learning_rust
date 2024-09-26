struct Customer {
    age: Option<i32>,
    email: String,
}

struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn main() {
    let mark = Customer {
        age: Some(32),
        email: "mark@gmail.com".to_owned(),
    };

    let mfon = Customer {
        age: None,
        email: "mfon@gmail.com".to_owned(),
    };

    match mfon.age {
        Some(age) => println!("Customer is {:?} years old", age),
        None => println!("Customer age not provided"),
    }

    // survey program

    let response = Survey {
        q1: Some(12),
        q2: Some(true),
        q3: None,
    };

    match response.q1 {
        Some(ans) => println!("q1: {:?}", ans),
        None => println!("q1: no response"),
    }
}
