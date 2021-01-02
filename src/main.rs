mod player;
mod strategies;
mod tournament;

use tournament::StrategySignature;
use tournament::Tournament;

fn main() {
    let configs: Vec<(String, StrategySignature)> = vec![
        ("Cooperator".to_string(), Box::new(strategies::cooperator)),
        ("Defector".to_string(), Box::new(strategies::defector)),
        ("AlternatorC".to_string(), Box::new(strategies::alternator_cooperator)),
        ("AlternatorD".to_string(), Box::new(strategies::alternator_defector)),
        ("TitForTat".to_string(), Box::new(strategies::tit_for_tat)),
        ("RandomCoinFlip".to_string(), Box::new(strategies::random_coin_flip)),
    ];
    let mut tournament = Tournament::new(configs, 1000);
    tournament.run();
    tournament.show_scores();
}
