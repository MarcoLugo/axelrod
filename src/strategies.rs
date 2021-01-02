use crate::player::Choice;
use fastrand::Rng;

pub fn cooperator(_: &[Choice], _: &[Choice], _: &mut Rng) -> Choice {
    Choice::Cooperate
}

pub fn defector(_: &[Choice], _: &[Choice], _: &mut Rng) -> Choice {
    Choice::Defect
}

pub fn random_coin_flip(_: &[Choice], _: &[Choice], rng: &mut Rng) -> Choice {
    if rng.bool() {
        Choice::Cooperate
    } else {
        Choice::Defect
    }
}

// Rapoport [1984]
pub fn tit_for_tat(_: &[Choice], other_choices: &[Choice], _: &mut Rng) -> Choice {
    match other_choices.last() {
        None => Choice::Cooperate,
        Some(&c) => c,
    }
}

// Axelrod [1984]
pub fn tit_for_two_tats(_: &[Choice], other_choices: &[Choice], _: &mut Rng) -> Choice {
    if other_choices
        .iter()
        .rev()
        .take(2)
        .filter(|&&c| c == Choice::Defect)
        .count()
        == 2
    {
        Choice::Defect
    } else {
        Choice::Cooperate
    }
}

// Friedman [1984]
pub fn grim_trigger(_: &[Choice], other_choices: &[Choice], _: &mut Rng) -> Choice {
    if other_choices.iter().any(|&c| c == Choice::Defect) {
        Choice::Defect
    } else {
        Choice::Cooperate
    }
}
