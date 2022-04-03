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
    direction: String
}

#[derive(Serialize, Deserialize)]
struct Item {
    name: String,
    description: String,
    location: i32
}

#[derive(Serialize, Deserialize)]
struct Verb {
    word: String,
    letter: Option<String>,
    objects: i32
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
            io::stdout().flush();
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
    for room in game.rooms.iter() {
        if room.number == room_number {
            for path in room.paths.iter() {
                for direction in directions.iter() {
                    if direction.contains(&&*input) {
                        if path.direction == direction[0] {
                            return path.destination
                        }
                    }
                }
            }
        }
    }
    // Built-in verbs
    // Examine, get/take, drop, use
    // Custom verbs
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