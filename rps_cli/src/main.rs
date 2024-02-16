use std::collections::HashMap;
use std::io;
use rand::{Rng};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Moves {
    ROCK,
    PAPER,
    SCISSORS,
}

struct Player {
    score: i32,
    moves: Vec<Moves>,
}

impl Player {
    fn make_random_move(&self) -> Moves {
        match rand::thread_rng().gen_range(1..=3) {
            1 => Moves::ROCK,
            2 => Moves::PAPER,
            3 => Moves::SCISSORS,
            _ => unreachable!(),
        }
    }

    fn add_move(&mut self, play: Moves) {
        self.moves.insert(0, play);
    }

    fn make_random_play(&mut self) {
        let random_move = self.make_random_move();
        self.add_move(random_move);
    }

    fn make_n_generated_moves(&mut self, n: i32) {
        for _ in 0..n {
            self.make_random_play();
        }
    }

    fn take_user_input_and_add(&mut self) {
        println!("Enter 1 for Rock, 2 for Paper and 3 for Scissors");

        loop {
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            let guess: i32 = match guess.trim().parse() {
                Ok(number) => number,
                Err(_) => {
                    println!("Enter either 1, 2, or 3");
                    continue;
                }
            };

            match guess {
                1 => self.add_move(Moves::ROCK),
                2 => self.add_move(Moves::PAPER),
                3 => self.add_move(Moves::SCISSORS),
                _ => continue
            }
            break;
        }
    }
}

fn main() {
    let mut computer = Player { score: 0, moves: Vec::new() };
    let mut user = Player { score: 0, moves: Vec::new() };
    let rounds = 3;

    let mut win_conditions: HashMap<(Moves, Moves), (u8, u8)> = HashMap::new();
    win_conditions.insert((Moves::ROCK, Moves::PAPER), (0, 1));
    win_conditions.insert((Moves::PAPER, Moves::ROCK), (1, 0));
    win_conditions.insert((Moves::ROCK, Moves::SCISSORS), (1, 0));
    win_conditions.insert((Moves::SCISSORS, Moves::ROCK), (0, 1));
    win_conditions.insert((Moves::PAPER, Moves::SCISSORS), (0, 1));
    win_conditions.insert((Moves::SCISSORS, Moves::PAPER), (1, 0));

    for _ in 0..rounds {
        computer.make_n_generated_moves(1);
        user.take_user_input_and_add();

        if let Some(result) = evaluate_move(&user.moves[0], &computer.moves[0], &win_conditions) {
            match result {
                (1, 0) => {
                    user.score += 1;
                    println!("You won the round!")
                },
                (0, 1) => {
                    computer.score += 1;
                    println!("The computer won the round!")
                },
                _ => {}
            }
        }else {
            user.score += 1;
            computer.score += 1;
            println!("Drew round!");
        }
        println!("Your score: {}, Computer's score: {}", user.score, computer.score);
        if user.score > computer.score {
            println!("Congratulations, you win!");
        }else if user.score < computer.score {
            println!("You lost, better luck next time!");
        }else{
            println!("Drew the match, can you do better?");
        }

        user.moves.clear();
        computer.moves.clear();
    }

}

fn evaluate_move<'a>(
    move1: &'a Moves,
    move2: &'a Moves,
    win_conditions: &'a HashMap<(Moves, Moves), (u8, u8)>,
) -> Option<&'a (u8, u8)> {
    win_conditions.get(&(*move1, *move2))
}
