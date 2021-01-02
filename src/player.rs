use crate::tournament::StrategySignature;

#[derive(Clone, Copy, Debug)]
pub enum Choice {
    Defect,
    Cooperate,
}

pub struct Player {
    strategy: StrategySignature,
    pub name: String,
}

impl Player {
    pub fn new(strategy: StrategySignature, name: String) -> Self {
        Self { strategy, name }
    }

    pub fn choose(&self, my_choices: &[Choice], other_choices: &[Choice]) -> Choice {
        (self.strategy)(my_choices, other_choices)
    }
}
