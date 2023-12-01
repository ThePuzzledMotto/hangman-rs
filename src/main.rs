use std::io;
// use rand::Rng;

fn main() {

    let mut player = Player::new();
    let word = get_word(); // Word to guess
    let mut won = false;
    let mut incorrect_chars = String::new();

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear

        if player.lives <= 0 {
            println!("Game Over, You Lose!");
            break;
        }
        else if won {
            println!("You Win!");
            break;
        }

        println!("HANGMAN-RS");
        drawing_hangman(&player);

        print_word(&word, &player);
        print!("Incorrect: ");
        for c in incorrect_chars.chars() {
            print!(" {},", c);
        }
        println!();
 

        player.get_guess();

        if !validate_guess(&word, &player) {
            player.lives -= 1;
            incorrect_chars.push(player.guesses.pop().unwrap());
        }
        
        won = check_win(&word, &player);
    }
}

struct Player {
    lives: i8,
    guesses: String,
}

impl Player {
    fn new() -> Self {
       Self {
            lives: 6,
            guesses: String::new(),
        } 
    }

    fn get_guess(&mut self) {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get input");

        guess = guess.to_lowercase();

        let guess: char = match guess.trim().parse() {
            Ok(c) => c,
            Err(_) => panic!("Guess input was not a char!"),
        };

        self.guesses.push(guess);    
    }
}

fn get_word() -> String {
    "universidade".to_string()
}

fn print_word(word: &str, player: &Player) {
    let mut tmp: bool;

    for c in word.chars() {
        tmp = false;
        for g in player.guesses.chars() {
            if c == g {
                print!(" {} ", c);
                tmp = true;
                break;
            }
        } 
        if tmp == false {
            print!(" _ ");
        }
    }
    println!("\n");
}



fn validate_guess(word: &str, p: &Player) -> bool {
    for c in p.guesses.chars() {
        if !word.contains(c) {
            return false; // Return false if a guessed character is not in the word
        }
    }
    return true; // All guessed characters are present in the word
}

fn check_win(word: &str, p: &Player) -> bool {
    for c in word.chars() {
        if !p.guesses.contains(c) {
            return false; // If any character is not guessed, return false
        }
    }
    return true; // All characters are guessed correctly
}


fn drawing_hangman(p: &Player) {
if p.lives == 6 {
println!(r" 
  ___
 |/  |
 |
 |
 |
/ \
");
} 
else if p.lives == 5 {
println!(r"
  ___
 |/  |
 |   O
 |  
 |
/ \
");
}
else if p.lives == 4 {
println!(r"
  ___
 |/  |
 |   O
 |   |
 |
/ \
")
}
else if p.lives == 3 {
println!(r"
  ___
 |/  |
 |   O
 |  /|
 |
/ \
");
}
else if p.lives == 2 {
println!(r"
  ___
 |/  |
 |   O
 |  /|\
 |  
/ \
");
}
else if p.lives == 1 {
println!(r"
  ___
 |/  |
 |   O
 |  /|\
 |  / 
/ \
");
}
else if p.lives == 0 {
println!(r"
  ___
 |/  |
 |   O
 |  /|\
 |  / \ 
/ \
");
}
}

