use std::convert::TryFrom;

use crate::player::{Choice, Player};

use contracts::*;
use fnv::FnvHashMap;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use itertools::Itertools;

pub type StrategySignature = Box<dyn Fn(&[Choice], &[Choice]) -> Choice>;

pub struct Tournament {
    n_iterations: u64,
    pub players: Vec<Player>,
    pub pairings: Vec<(usize, usize)>,
    history: FnvHashMap<(usize, usize), (Vec<Choice>, Vec<Choice>)>,
    scores: Vec<i32>,
}

impl Tournament {
    #[requires(configs.len() % 2 == 0)]
    #[requires(n_iterations > 0)]
    pub fn new(configs: Vec<(&'static str, StrategySignature)>, n_iterations: u64) -> Self {
        let n_players = configs.len();

        let mut tournament = Self {
            n_iterations,
            players: configs.into_iter().map(|c| Player::new(c.1, c.0)).collect(),
            pairings: (0..n_players)
                .combinations_with_replacement(2)
                .map(|x| (x[0], x[1]))
                .collect(),
            history: FnvHashMap::default(),
            scores: vec![0; n_players],
        };

        let n_iterations =
            usize::try_from(n_iterations).expect("n_iterations could not be converted to usize.");
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
            let history = self.history.get(pairing).unwrap();

            let choice_a = player_a.choose(&history.0, &history.1);
            let choice_b = player_b.choose(&history.1, &history.0);

            // store results
            let h = self.history.get_mut(pairing).unwrap();
            h.0.push(choice_a);
            h.1.push(choice_b);
        }
    }

    pub fn run(&mut self) {
        println!("[+] Running tournament...");
        let pb = ProgressBar::new(self.n_iterations as u64);
        pb.set_draw_delta(self.n_iterations as u64 / 200);
        pb.set_style(ProgressStyle::default_bar().template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        ));

        for _ in (0..self.n_iterations).progress_with(pb) {
            self.run_iteration();
        }
    }

    pub fn show_scores(&mut self) {
        println!("[+] Match scores:");

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

            println!(
                "Match: {} ({}) vs {} ({}) -> ({}, {})",
                pairing.0,
                self.players[pairing.0].name,
                pairing.1,
                self.players[pairing.1].name,
                score_a,
                score_b
            );
        }

        println!("[+] Total scores:");

        for (i, score) in self.scores.iter().enumerate() {
            println!("{} ({}): {} points.", i, self.players[i].name, score);
        }
    }
}
