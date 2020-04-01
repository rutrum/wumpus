mod cave;
mod player;
mod prompt;
use player::Player;

use cave::Cave;
use prompt::Action;

use std::io;
use colored::*;

fn main() {
    let num_rooms = 10;
    let num_hazards = 3;
    let dungeon = match Cave::new(num_rooms, num_hazards) {
        Ok(cave) => cave,
        Err(s) => {
            println!("{}", s);
            return;
        }
    };

    let mut p = Player::new(3, 0);

    println!("{}", "Welcome to Enumpus!".bright_green());
    println!("A lot of {} did this.", "hard work".italic());
    println!();
    println!("{}", p);

    loop {
        dungeon.print_room(&p);

        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!();

        match Action::from(input) {
            Ok(a) => match a {
                Action::Move(next) => {
                    p = match dungeon.go_to_room(p, next) {
                        Ok(new) => new,
                        Err((s, new)) => {
                            println!("{}", s.bright_red());
                            new
                        }
                    }
                }

                Action::Shoot(at) => {
                    p = match dungeon.shoot_at_room(p, at) {
                        Ok(new) => new,
                        Err((s, new)) => {
                            println!("{}", s.bright_red());
                            new
                        }
                    }
                }

                Action::Help => {
                    println!("Type 'move 4' to move, or 'shoot 4' to shoot into room 4.  To quit type just that.");
                }

                Action::Quit => {
                    break;
                }
            },
            Err(s) => {
                println!("{}", s.bright_red()); 
            }
        }

        println!("{}", p);

        if let Player::Dead(_) | Player::Win = p {
            break;
        }
    }

    println!("\nThanks for playing!");
}
