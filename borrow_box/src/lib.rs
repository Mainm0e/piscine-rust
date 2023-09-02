// todo: borrow box
/* 
Instructions
Game time.

You will implement some CRUD functionality for a game session. You will need to implement the GameSession structure with the following associated functions:

new: which initializes a game session state with player names and some other information. This function returns the structure wrapped in a Box.

read_winner: which returns a tuple with the name and score of the player who is currently winning. In the case that no player is winning, it should return the same tuple with the string "Same score! tied" and the tied score.

update_score: which receives the name of a player, and increments their score. This function should do nothing if the the game session is already finished or if the name received doesn't match any player.

delete: which takes ownership of the boxed game session and returns a string: "game deleted: id -> 0", where 0 is the id of the GameSession.

If nb_games is 5, then it is "best out of 5", and no more than 5 games can be played. If some player has a score of 3, then the game session is also finished. This is because there is an insufficient number of remaining games for the trailing player to catch up.

Expected Functions
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {

    }
    pub fn read_winner(&self) -> (String, u16) {

    }
    pub fn update_score(&mut self, user_name: String) {

    }
    pub fn delete(self) -> String {

    }
}
*/

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16
    
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        let game = GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games
        };

        Box::new(game)

    }

    pub fn read_winner(&self) -> (String, u16) {
        let (p1_name, p1_score) = &self.p1;
        let (p2_name, p2_score) = &self.p2;
        
        println!("game: {:?}", self);
        if self.nb_games == *p1_score + *p2_score {
          if *p1_score < *p2_score{
                return (p2_name.clone(), *p2_score)
            } else if *p1_score > *p2_score {
                return (p1_name.clone(), *p1_score)
          }
        }

            if p1_score > p2_score {
                (p1_name.clone(), *p1_score)
            } else if p2_score > p1_score {
                (p2_name.clone(), *p2_score)
            } else {
                ("Same score! tied".to_string(), *p1_score)
            }
    }
    
    pub fn update_score(&mut self, user_name: String) {
        let (p1_name, p1_score) = &mut self.p1;
        let (p2_name, p2_score) = &mut self.p2;

        // check if game is finished
        if self.nb_games == *p1_score + *p2_score {
            return
        } else if *p1_score == 3 || *p2_score == 3 {
            return
        }

        if user_name == *p1_name {
            *p1_score += 1;
        } else if user_name == *p2_name {
            *p2_score += 1;
        }


    }

    pub fn delete(self) -> String {

        let id = self.id;
        format!("game deleted: id -> {}", id)

    }
}