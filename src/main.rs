mod player;
mod strategies;
mod tournament;

use tournament::StrategySignature;
use tournament::Tournament;

fn main() {
    let configs: Vec<(String, StrategySignature)> = vec![
        ("Cooperator".to_string(), Box::new(strategies::cooperator)),
        ("Defector".to_string(), Box::new(strategies::defector)),
    ];
    let mut tournament = Tournament::new(configs, 1);
    tournament.run(true);
    tournament.show_scores();
}
