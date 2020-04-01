use colored::*;

/// This is a player
pub enum Player {
    Alive { arrows: usize, location: usize },
    Dead(String),
    Win,
}

impl Player {
    // Creates a new player with a arrows at loc
    pub fn new(a: usize, loc: usize) -> Player {
        Player::Alive {
            arrows: a,
            location: loc,
        }
    }

    // Get player's location.
    pub fn get_loc(&self) -> Result<usize, String> {
        match self {
            Player::Alive { location: l, .. } => Ok(*l),
            Player::Dead(_) => Err("Player is dead!".to_string()),
            Player::Win => Err("The player has won!".to_string()),
        }
    }

    // Decrements player's arrows by one.  If 0 arrows left,
    // player is dead, if it isn't already.
    pub fn shoot(self) -> Player {
        if let Player::Alive { arrows: a, location: l } = self {
            if a > 1 {
                Player::new(a - 1, l)
            } else {
                Player::Dead("You have ran out of arrows. A grue comes to eat you.".bold().to_string())
            }
        } else {
            self
        }
    }

    // Moves a player to new location.
    pub fn move_to(self, new: usize) -> Player {
        if let Player::Alive { arrows: a, .. } = self {
            Player::new(a, new)
        } else {
            self
        }
    }

    // Destroys current player and replaces it with a dead one
    pub fn kill(self, msg: String) -> Player {
        Player::Dead(msg)
    }

    // Returns a player who has killed the Wumpus!
    pub fn praise(self) -> Player {
        Player::Win
    }
}

use std::fmt;

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Player::*;
        match self {
            Alive {
                arrows: a,
                location: l,
            } => write!(f, "You are in room {}.  You have {} arrows.", l, a),
            Dead(s) => write!(f, "{}\nYou have died.", s),
            Win => write!(f, "{}", "You've killed the Wumpus!".bold().cyan()),
        }
    }
}
