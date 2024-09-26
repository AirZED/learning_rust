// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(String, f32),
    Vip(String, f32),
    Standard(f32),
}

// struct Ticket {
//     price: f32,
//     name: String,
// }

fn main() {
    let tickets = vec![
        Ticket::Backstage("eno".to_owned(), 32.8),
        Ticket::Vip("eno".to_owned(), 32.8),
        Ticket::Standard(32.8)
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(name, price) => {
                println!("This is a Backstage ticket with name {:?} and price {:?}", name, price);
            }
            _ => (),
        }
    }
}
