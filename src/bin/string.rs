struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn main() {
    let reciept = vec![
        LineItem { name: "cereal".to_owned(), count: 40 },
        LineItem { name: String::from("Fruit"), count: 50 }
    ];

    for item in reciept {
        print_name(&item.name);
        println!("name: {:?} count {:?}", item.name, item.count);
    }
}
