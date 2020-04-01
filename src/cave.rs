use rand::prelude::*;
use std::collections::HashSet;

use crate::player::Player;

use colored::*;

pub struct Cave {
    rooms: Vec<Room>,
}

impl Cave {
    pub fn new(t: usize, h: usize) -> Result<Cave, String> {
        if t < 5 {
            return Err("too few rooms to make cave".to_string());
        } else if h + 2 >= t {
            // plus two: starting location and wumpus
            return Err("too many hazards for the number of rooms".to_string());
        }

        // First create rooms
        let mut rooms: Vec<Room> = Vec::new();
        for i in 0..t {
            let fst = (i + 1) % t;
            let snd = (i + 1 + t / 4) % t;
            let trd = (i + 1 + 3 * t / 4) % t;
            let adj = [fst, snd, trd];

            let room = Room::new(adj);
            rooms.push(room);
        }

        // Then assign hazards
        Cave::assign_hazards(&mut rooms, h);

        Ok(Cave { rooms: rooms })
    }

    fn assign_hazards(rooms: &mut Vec<Room>, h: usize) {
        let mut random = rand::thread_rng();

        // first wumpus
        let num = random.gen_range(1, rooms.len());
        rooms[num].add_hazard(Hazard::Wumpus).unwrap();

        // Then add either bats or pitfalls randomly
        // up to h hazards
        for _ in 0..h {
            let hazard = if random.gen::<bool>() {
                Hazard::Pitfall
            } else {
                Hazard::Bats
            };

            loop {
                let num = random.gen_range(1, rooms.len());
                if let Ok(_) = rooms[num].add_hazard(hazard) {
                    break;
                }
            }
        }
    }

    fn valid_room(&self, player: &Player, num: usize) -> bool {
        let current = player.get_loc().unwrap();
        self.rooms[current].is_adjacent(num)
    }

    pub fn go_to_room(&self, player: Player, next: usize) -> Result<Player, (String, Player)> {
        if !self.valid_room(&player, next) {
            return Err(("That room isn't connected to here!".to_string(), player));
        }

        Ok(self.checkout_room(player, next))
    }

    // Trys to move player into next, player may die from hazards
    fn checkout_room(&self, player: Player, next: usize) -> Player {
        if let Some(hazard) = self.rooms[next].hazard {
            match hazard {
                Hazard::Wumpus => player.kill("You are eaten by the wumpus!".bold().to_string()),
                Hazard::Pitfall => player.kill("You slip into a pit!".bold().to_string()),
                Hazard::Bats => {
                    // Pick random assignment of room and run this function again
                    let mut random = rand::thread_rng();
                    let new = random.gen_range(0, self.rooms.len());
                    println!("{}", "You are lifted up by bats!".bold().blue());
                    println!("You fall into room {}.\n", new);
                    self.checkout_room(player, new)
                }
            }
        } else {
            player.move_to(next)
        }
    }

    pub fn shoot_at_room(&self, player: Player, at: usize) -> Result<Player, (String, Player)> {
        if !self.valid_room(&player, at) {
            return Err(("that room isn't connect to here".to_string(), player));
        }

        if let Some(Hazard::Wumpus) = self.rooms[at].hazard {
            Ok(player.praise())
        } else {
            println!("You don't appear to have shot the Wumpus.");
            Ok(player.shoot())
        }
    }

    pub fn print_room(&self, player: &Player) {
        if let Player::Alive { location: l, .. } = player {
            let room = &self.rooms[*l];
            println!("{}", room);

            let mut hazards = HashSet::new();
            for &adj in &room.adjacent {
                if let Some(h) = self.rooms[adj].hazard {
                    hazards.insert(h);
                }
            }
            for h in hazards {
                println!("{}", h);
            }
        }
    }
}

struct Room {
    adjacent: [usize; 3],
    hazard: Option<Hazard>,
}

impl Room {
    fn new(adj: [usize; 3]) -> Room {
        Room {
            adjacent: adj,
            hazard: None,
        }
    }

    fn add_hazard(&mut self, h: Hazard) -> Result<(), String> {
        match self.hazard {
            None => {
                self.hazard = Some(h);
                Ok(())
            }
            Some(_) => Err("already a hazard in this room".to_string()),
        }
    }

    pub fn is_adjacent(&self, num: usize) -> bool {
        self.adjacent.contains(&num)
    }
}

use std::fmt;

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "There are tunnels to {}, {}, and {} from here.",
            self.adjacent[0], self.adjacent[1], self.adjacent[2],
        )
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Hazard {
    Wumpus,
    Bats,
    Pitfall,
}

impl fmt::Display for Hazard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hazard::Wumpus => write!(f, "{}", "There is a wumpus nearby!".italic()),
            Hazard::Bats => write!(f, "You hear screaching."),
            Hazard::Pitfall => write!(f, "Your voice echos."),
        }
    }
}
