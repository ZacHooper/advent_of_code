use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let opponent_choice = rng.gen_range(0..3);
    let player_result = 1;
    let game = Game::new(&opponent_choice, &player_result);
    println!("I chose {:?}", game.player);
    println!("You chose {:?}", game.opponent);
    match game.result() {
        Result::Win => println!("You win!"),
        Result::Lose => println!("You lose!"),
        Result::Tie => println!("You tied!"),
    }
    println!("Your score is {}", game.total_score());
}

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn beats(&self, other: &Choice) -> bool {
        match (self, other) {
            (Choice::Rock, Choice::Scissors) => true,
            (Choice::Paper, Choice::Rock) => true,
            (Choice::Scissors, Choice::Paper) => true,
            _ => false,
        }
    }
}

fn get_choice(choice: &i32) -> Choice {
    match choice {
        0 => Choice::Rock,
        1 => Choice::Paper,
        2 => Choice::Scissors,
        _ => unreachable!(),
    }
}

fn get_result_choice(opp_choice: Choice, wanted_result: i32) -> Choice {
    match (opp_choice, wanted_result) {
        (Choice::Rock, 0) => Choice::Rock,
        (Choice::Rock, 1) => Choice::Paper,
        (Choice::Rock, 2) => Choice::Scissors,
        (Choice::Paper, 0) => Choice::Paper,
        (Choice::Paper, 1) => Choice::Scissors,
        (Choice::Paper, 2) => Choice::Rock,
        (Choice::Scissors, 0) => Choice::Scissors,
        (Choice::Scissors, 1) => Choice::Rock,
        (Choice::Scissors, 2) => Choice::Paper,
        _ => unreachable!(),
    }
}

struct Game {
    opponent: Choice,
    player: Choice,
}

impl Game {
    fn new(opp: &i32, player: &i32) -> Game {
        Game {
            opponent: get_choice(opp),
            player: get_result_choice(get_choice(&opp), *player),
        }
    }

    fn result(&self) -> Result {
        if self.player.beats(&self.opponent) {
            Result::Win
        } else if self.opponent.beats(&self.player) {
            Result::Lose
        } else {
            Result::Tie
        }
    }

    fn choice_score(&self) -> i32 {
        match &self.player {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn result_score(&self) -> i32 {
        match self.result() {
            Result::Win => 6,
            Result::Lose => 0,
            Result::Tie => 3,
        }
    }

    fn total_score(&self) -> i32 {
        self.choice_score() + self.result_score()
    }
}

enum Result {
    Win,
    Lose,
    Tie,
}
