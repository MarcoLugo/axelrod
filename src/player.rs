use crate::tournament::StrategySignature;

#[derive(Clone, Copy, Debug)]
pub enum Choice {
    Defect,
    Cooperate,
}

pub struct Player {
    strategy: StrategySignature,
    pub name: &'static str,
}

impl Player {
    pub fn new(strategy: StrategySignature, name: &'static str) -> Self {
        Self { strategy, name }
    }

    pub fn choose(&self, my_choices: &[Choice], other_choices: &[Choice]) -> Choice {
        (self.strategy)(my_choices, other_choices)
    }
}
