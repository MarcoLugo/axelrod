use crate::player::Choice;
use fastrand::Rng;

pub fn cooperator(_: &[Choice], _: &[Choice], _: &mut Rng) -> Choice {
    Choice::Cooperate
}

pub fn defector(_: &[Choice], _: &[Choice], _: &mut Rng) -> Choice {
    Choice::Defect
}

//Axelrod, R. (1980). Effective Choice in the Prisoner’s Dilemma. Journal of Conflict Resolution, 24(1), 3–25. doi:10.1177/002200278002400101
// (RANDOM)
pub fn random_coin_flip(_: &[Choice], _: &[Choice], rng: &mut Rng) -> Choice {
    if rng.bool() {
        Choice::Cooperate
    } else {
        Choice::Defect
    }
}

//Axelrod, R. (1980). Effective Choice in the Prisoner’s Dilemma. Journal of Conflict Resolution, 24(1), 3–25. doi:10.1177/002200278002400101
// (Anatol Rapoport)
pub fn tit_for_tat(_: &[Choice], other_choices: &[Choice], _: &mut Rng) -> Choice {
    match other_choices.last() {
        None => Choice::Cooperate,
        Some(&c) => c,
    }
}

// Axelrod, Robert (1984). The Evolution of Cooperation. Basic Books. ISBN 978-0-465-02121-5
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

//Axelrod, R. (1980). Effective Choice in the Prisoner’s Dilemma. Journal of Conflict Resolution, 24(1), 3–25. doi:10.1177/002200278002400101
// (FRIEDMAN)
pub fn grim_trigger(_: &[Choice], other_choices: &[Choice], _: &mut Rng) -> Choice {
    if other_choices.iter().any(|&c| c == Choice::Defect) {
        Choice::Defect
    } else {
        Choice::Cooperate
    }
}
