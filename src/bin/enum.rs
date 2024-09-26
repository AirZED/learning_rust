enum Mouse {
    LeftClick,
    RightClick,
    Scroll(i32),
    Drag(i32, i32),
}

enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: f32,
}

fn main() {
    let n = 4;

    match n {
        3 => println!("Three"),
        other => println!("the number is {:?}", other),
    }

    let flat = Discount::Flat(9);
    match flat {
        Discount::Flat(2) => println!("The flat amount is 2"),
        Discount::Flat(amount) => println!("Flat discount is {:?}", amount),
        _ => (),
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 40.0,
    };

    match concert {
        Ticket { price: 50.0, event } => println!("The event @ 50 = {:?}", event),
        Ticket { price, .. } => println!("Price = {:?}", price),
    }
}
