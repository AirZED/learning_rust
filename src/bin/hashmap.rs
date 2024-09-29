use std::collections::HashMap;


struct Contents {
    content: String,
}

fn main() {
    let mut lockers = HashMap::new();

    lockers.insert(1, Contents { content: "rice".to_owned() });
    lockers.insert(7, Contents { content: "beans".to_owned() });
    lockers.insert(8, Contents { content: "shirt".to_owned() });
    lockers.insert(6, Contents { content: "roasted corn".to_owned() });

    match &lockers.get(&6) {
        Some(content) => println!("{:?}", content.content),
        None => println!("Content not found for key"),
    }

    for (key, content) in lockers.iter() {
        println!("{:?}", content.content);
    }
}
