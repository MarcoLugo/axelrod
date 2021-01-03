use crate::player::{Choice, Player};
use contracts::*;
use fastrand::Rng;
use fnv::FnvHashMap;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use itertools::Itertools;
use std::convert::TryFrom;

pub type StrategySignature = Box<dyn Fn(&[Choice], &[Choice], &mut Rng) -> Choice>;

pub struct Tournament {
    n_iterations: u64,
    players: Vec<Player>,
    pairings: Vec<(usize, usize)>,
    history: FnvHashMap<(usize, usize), (Vec<Choice>, Vec<Choice>)>,
    scores: Vec<i32>,
}

impl Tournament {
    #[requires(configs.len() > 0)]
    #[requires(n_iterations > 0)]
    pub fn new(configs: Vec<(&'static str, StrategySignature)>, n_iterations: u64) -> Self {
        let n_players = configs.len();

        let mut tournament = Self {
            n_iterations,
            players: configs
                .into_iter()
                .map(|c| Player::new(c.1, c.0, create_seed_from_str(c.0)))
                .collect(),
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

    fn run_iteration(&mut self) {
        for pairing in &self.pairings {
            let history = self.history.get_mut(pairing).unwrap();

            let player_a = &mut self.players[pairing.0];
            let choice_a = player_a.choose(&history.0, &history.1);

            let player_b = &mut self.players[pairing.1];
            let choice_b = player_b.choose(&history.1, &history.0);

            // store results
            history.0.push(choice_a);
            history.1.push(choice_b);
        }
    }

    pub fn run(&mut self) {
        println!("[+] Running tournament...");
        let pb = ProgressBar::new(self.n_iterations);
        pb.set_draw_delta(self.n_iterations / 200);
        pb.set_style(ProgressStyle::default_bar().template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        ));

        for _ in (0..self.n_iterations).progress_with(pb) {
            self.run_iteration();
        }
    }

    pub fn show_scores(&mut self) {
        self.show_pairing_scores();
        self.show_total_scores();
    }

    fn compute_payoffs(choice_a: &Choice, choice_b: &Choice) -> (i32, i32) {
        match (choice_a, choice_b) {
            (Choice::Cooperate, Choice::Cooperate) => (3, 3),
            (Choice::Cooperate, Choice::Defect) => (0, 5),
            (Choice::Defect, Choice::Cooperate) => (5, 0),
            (Choice::Defect, Choice::Defect) => (1, 1),
        }
    }

    #[requires(choices_a.len() == choices_b.len())]
    fn compute_pairing_scores(choices_a: &[Choice], choices_b: &[Choice]) -> (i32, i32) {
        choices_a
            .iter()
            .zip(choices_b)
            .fold((0, 0), |(acc_a, acc_b), (a, b)| {
                let (result_a, result_b) = Tournament::compute_payoffs(a, b);
                (acc_a + result_a, acc_b + result_b)
            })
    }

    fn show_pairing_scores(&mut self) {
        println!("\n[+] Round-robin scores:");
        println!(
            "{0:>4} | {1:>25} | {2:>7} | {3:<7} | {4:<25} | {5:<4}",
            "Id 1", "Player 1", "Score 1", "Score 2", "Player 2", "Id 2"
        );

        for pairing in &self.pairings {
            let (choices_a, choices_b) = self.history.get(pairing).unwrap();
            let (score_a, score_b) = Tournament::compute_pairing_scores(choices_a, choices_b);

            self.scores[pairing.0] += score_a;
            self.scores[pairing.1] += score_b;
            let name_a = self.players[pairing.0].name;
            let name_b = self.players[pairing.1].name;

            println!(
                "{:>4} | {:>25} | {:>7} | {:<7} | {:<25} | {:<4}",
                pairing.0, name_a, score_a, score_b, name_b, pairing.1
            );
        }
    }

    fn show_total_scores(&self) {
        println!("\n[+] Ranked total scores:");
        println!(
            "{0:>5} | {1:>25} | {2:>10} | {3:>10}",
            "id", "name", "total", "average"
        );

        let mut player_indices_sorted_by_score: Vec<(_, _)> = self
            .scores
            .iter()
            .enumerate()
            .map(|(i, &s)| (i, s))
            .collect();
        player_indices_sorted_by_score.sort_by(|a, b| b.1.cmp(&a.1));

        for (i, score) in player_indices_sorted_by_score {
            let name = self.players[i].name;
            let average_score = score as f64 / self.n_iterations as f64;
            println!(
                "{:>5} | {:>25} | {:>10} | {:>10}",
                i, name, score, average_score
            );
        }
    }
}

fn create_seed_from_str(s: &str) -> u64 {
    s.as_bytes().iter().fold(0, |acc, &x| acc + x as u64)
}
