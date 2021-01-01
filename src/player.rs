use crate::tournament::StrategySignature;

#[derive(Clone, Copy, Debug)]
pub enum Choice {
    Defect,
    Cooperate,
}

pub struct Player {
    id: usize,
    strategy: StrategySignature,
    pub name: String,
}

impl Player {
    pub fn new(id: usize, strategy: StrategySignature, name: String) -> Self {
        Self {
            id,
            strategy,
            name,
        }
    }

    pub fn choose(&self) -> Choice {
        (self.strategy)()
    }
}
