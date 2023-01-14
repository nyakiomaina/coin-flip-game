use std::io;

enum Coin {
    Heads,
    Tails,
}

fn flip_coin() -> Coin {
    if rand::random() {
        Coin::Heads
    } else{
        Coin::Tails
    }
}

fn play_game() {
    println!("Welcome to coin flip game!");
    println!("Guess the outcome of the cin flip game (Heads ro Tails): ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess = guess.trim();
    let coin_flip = flip_coin();
    let coin_flip_str = match coin_flip {
        Coin::Heads => "Heads",
        Coin::Tails => "Tails",
    };

    if guess == coin_flip_str {
        println!("You Win!");
    }else {
        println!("You lose! The coin landed on {}", coin_flip_str);
    }
}

fn display_stats(wins: u32, losses: u32) {
    println!("\n\nNumber of wins: {}", wins);
    println!("\n\nNumber of losses is: {}", losses);
}
fn main() {
    let mut wins = 0;
    let mut losses = 0;

    loop{
        play_game();
        println!("Do you want to play again? (yes/no)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read line");

        let play_again = play_again.trim();
        if play_again == "n" {
            break
        }
        wins +=1;
        losses +=1;
    }
    display_stats(wins, losses);
}
