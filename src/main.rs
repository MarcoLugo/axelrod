mod player;
mod strategies;
mod tournament;

use tournament::StrategySignature;
use tournament::Tournament;

fn main() {
    let configs: Vec<(&str, StrategySignature)> = vec![
        ("Cooperator", Box::new(strategies::cooperator)),
        ("Defector", Box::new(strategies::defector)),
        ("AlternatorC", Box::new(strategies::alternator_cooperator)),
        ("AlternatorD", Box::new(strategies::alternator_defector)),
        ("TitForTat", Box::new(strategies::tit_for_tat)),
        ("RandomCoinFlip", Box::new(strategies::random_coin_flip)),
    ];

    let mut tournament = Tournament::new(configs, 1000);
    tournament.run();
    tournament.show_scores();
}
