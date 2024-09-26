#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Menu choice was not found".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("Choice = {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}

fn main() {
    // let choice = get_choice("p");

    // match choice {
    //     Ok(inner_choice) => print_choice(&inner_choice),
    //     Err(e) => println!("error = {:?}", e),
    // }

    pick_choice("start");
}
