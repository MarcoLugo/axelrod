mod player;
mod strategies;
mod tournament;

use tournament::{StrategySignature, Tournament};

fn main() {
    let configs: Vec<(&str, StrategySignature)> = vec![
        ("Cooperator", Box::new(strategies::cooperator)),
        ("Defector", Box::new(strategies::defector)),
        ("TitForTat", Box::new(strategies::tit_for_tat)),
        ("TitForTwoTats", Box::new(strategies::tit_for_two_tats)),
        ("GrimTrigger", Box::new(strategies::grim_trigger)),
        ("RandomCoinFlip", Box::new(strategies::random_coin_flip)),
    ];

    let mut tournament = Tournament::new(configs, 100);
    tournament.run();
    tournament.show_scores();
}
