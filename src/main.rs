mod game;

fn main() {


    let mut game = game::new();

    while game.get_guesses() > 0 {
        
        println!("\n");
        if game.is_won() {
            println!("You Win! The word was {}.", game.get_word());
            std::process::exit(1);
        };

        println!("You have {} guesses left", game.get_guesses());
        game.show_progress();

        println!("Plesae pick a letter");
        game.accept_guess();
    }

    println!("You Lose...Better luck next time.");
}
