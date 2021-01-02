use crate::tournament::StrategySignature;
use fastrand::Rng;
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Choice {
    Defect,
    Cooperate,
}

pub struct Player {
    strategy: StrategySignature,
    pub name: &'static str,
    rng: Rng,
}

impl Player {
    pub fn new(strategy: StrategySignature, name: &'static str, seed: usize) -> Self {
        Self {
            strategy,
            name,
            rng: Rng::with_seed(u64::try_from(seed).expect("Seed could not be converted to u64.")),
        }
    }

    pub fn choose(&mut self, my_choices: &[Choice], other_choices: &[Choice]) -> Choice {
        (self.strategy)(my_choices, other_choices, &mut self.rng)
    }
}
