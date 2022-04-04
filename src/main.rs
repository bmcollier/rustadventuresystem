use std::io::{self, BufRead, Read, Write};
use serde::{Serialize, Deserialize};
use std::fs::File;
use toml;

const UNDERLINE: &str = "\x1B[4m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1B[0m";

#[derive(Serialize, Deserialize)]
struct Game {
    rooms: Vec<Room>,
    items: Vec<Item>,
    verbs: Vec<Verb>,
    name: String,
    introduction: String
}

#[derive(Serialize, Deserialize)]
struct Room {
    number: i32,
    name: String,
    description: String,
    paths: Vec<Path>
}

#[derive(Serialize, Deserialize)]
struct Path {
    destination: i32,
    direction: String,
    door: Option<bool>,
    locked: Option<bool>
}

#[derive(Serialize, Deserialize)]
struct Item {
    name: String,
    description: String,
    location: i32,
    portable: Option<bool>
}

#[derive(Serialize, Deserialize)]
struct Verb {
    word: String,
    letter: Option<String>,
    objects: i32
}

#[derive(Serialize, Deserialize)]
struct Action {
    verb: String,
    object: String,
    target: String,
    transport: i32,             // On action, transport player to this room
    swap_from: Option<i32>,     // On action, swap this room with another and print the new view.
    swap_to: Option<i32>,       // This is the room to swap in. Paths will be swapped in as well.
    unlock: Option<String>,     // On action, unlock the door in this direction
    destroy: Option<String>     // On action, destroy any items with this name
}

fn show_room(room_number: i32, game: &Game) {
    for room in game.rooms.iter() {
        if room.number == room_number {
            println!("{}{}{}", BOLD, room.name, RESET);
            println!("{}", room.description);
            for item in game.items.iter() {
                if item.location == room_number {
                    let mut n_optional = "";
                    if ["a","e","i","o","u", "A","E","I","O","U"].contains(
                        &&*item.name.chars().next().unwrap().to_string()) {
                        n_optional = "n";
                    }
                    println!("\nThere is a{} {} here.", n_optional, item.name);
                }
            }


            print!("> ");
            let result= io::stdout().flush();
            if result.is_err() {
                println!("Error flushing console.")
            }
            return;
        }
    }
    println!("Error in code.");
}

fn process_input(room_number: i32, game: &Game) -> i32 {
    let directions: Vec<Vec<&str>> = vec![
        vec!["North", "N", "north", "n"],
        vec!["South", "S", "south", "s"],
        vec!["West", "W", "west", "w"],
        vec!["East", "E", "east", "e"],
        vec!["Up", "U", "up", "u"],
        vec!["Down", "D", "down", "d"],
    ];
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut chose_direction = false;
    for room in game.rooms.iter() {
        if room.number == room_number {
            for direction in directions.iter() {
                if direction.contains(&&*input) {
                    chose_direction = true;
                    for path in room.paths.iter() {
                        if path.direction == direction[0] {
                            return path.destination
                        }
                    }
                }
            }
            if chose_direction {
                println!("Sorry, you cannot go that way.\n");
                return room_number;
            }
        }
    }
    // Get words from sentence
    let sentence: Vec<&str> = input.split(" ").collect();
    // Built-in verbs
    if ["inventory", "inv", "i"].contains(&sentence[0]) {
        // TODO: Show inventory
        return room_number;
    }
    if ["help", "h"].contains(&sentence[0]) {
        println!("There is no help\n");
        // TODO: Show help
        return room_number;
    }
    // Examine
    if ["examine", "x"].contains(&sentence[0]) {
        println!("There is no help\n");
        // TODO: Show help
        return room_number;
    }
    // Get or Take
    if ["get", "g", "take", "t"].contains(&sentence[0]) {
        println!("There is no help\n");
        // TODO: Show help
        return room_number;
    }
    // Drop
    if ["drop", "d"].contains(&sentence[0]) {
        println!("There is no help\n");
        // TODO: Show help
        return room_number;
    }
    // Custom verbs including "use"
    for verb in game.verbs.iter() {
        if verb.word == input {
            println!("You used {}", verb.word);
            return room_number;
        }
    }
    println!("I'm sorry, I didn't understand that.\n");
    return room_number;
}

fn main() {
    let mut room_number = 1;
    let mut file = File::open("adventure.toml").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let game: Game = toml::from_str(&*contents).unwrap();
    //let game_toml = toml::to_string(&game).unwrap();
    //println!("{}", game_toml);
    println!("\n{}{}{}", UNDERLINE, game.name, RESET);
    println!("{}\n", game.introduction);
    loop {
        show_room(room_number, &game);
        room_number = process_input(room_number, &game);
    }
}