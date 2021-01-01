use std::collections::HashMap;

use crate::{
    player::{Choice, Player}
};

use contracts::*;
use itertools::Itertools;

pub type StrategySignature = Box<dyn Fn() -> Choice>;

pub struct Tournament {
    n_iterations: usize,
    pub players: Vec<Player>,
    pub pairings: Vec<(usize, usize)>,
    history: HashMap<(usize, usize), (Vec<Choice>, Vec<Choice>)>,
    scores: Vec<i32>,
}

impl Tournament {
    #[requires(configs.len() % 2 == 0)]
    #[requires(n_iterations > 0)]
    pub fn new(configs: Vec<(String, StrategySignature)>, n_iterations: usize) -> Self {
        let n_players = configs.len();

        let mut tournament = Self {
            n_iterations,
            players: configs
                .into_iter()
                .enumerate()
                .map(|(i, c)| Player::new(i, c.1, c.0))
                .collect(),
            pairings: (0..n_players)
                .combinations_with_replacement(2)
                .map(|x| (x[0], x[1]))
                .collect(),
            history: HashMap::new(),
            scores: vec![0; n_players],
        };

        for history_pairing in &tournament.pairings {
            tournament.history.insert(
                *history_pairing,
                (
                    Vec::with_capacity(n_iterations),
                    Vec::with_capacity(n_iterations),
                ),
            );
        }

        tournament
    }

    fn compute_payoffs(player_a: &Choice, player_b: &Choice) -> (i32, i32) {
        match (player_a, player_b) {
            (Choice::Cooperate, Choice::Cooperate) => (3, 3),
            (Choice::Cooperate, Choice::Defect) => (0, 5),
            (Choice::Defect, Choice::Cooperate) => (5, 0),
            (Choice::Defect, Choice::Defect) => (1, 1),
        }
    }

    fn run_iteration(&mut self) {
        for pairing in &self.pairings {
            let player_a = &self.players[pairing.0];
            let player_b = &self.players[pairing.1];

            let choice_a = player_a.choose();
            let choice_b = player_b.choose();

            match self.history.get_mut(pairing) {
                Some(h) => {
                    h.0.push(choice_a);
                    h.1.push(choice_b);
                }
                None => unreachable!(),
            };
        }
    }

    pub fn run(&mut self, verbose: bool) {
        for _ in 0..self.n_iterations {
            self.run_iteration();
        }
    }

    pub fn show_scores(&mut self) {
        for pairing in &self.pairings {
            let (choices_a, choices_b) = self.history.get(pairing).unwrap();

            let (score_a, score_b) =
                choices_a
                    .iter()
                    .zip(choices_b)
                    .fold((0, 0), |(acc_a, acc_b), (a, b)| {
                        let (result_a, result_b) = Tournament::compute_payoffs(a, b);
                        (acc_a + result_a, acc_b + result_b)
                    });

            self.scores[pairing.0] += score_a;
            self.scores[pairing.1] += score_b;

            println!("Match: {:?} -> ({}, {})", pairing, score_a, score_b);
        }

        println!("Total scores:");

        for (i, score) in self.scores.iter().enumerate() {
            println!("{} ({}): {} points.", i, self.players[i].name, score);
        }
    }
}
