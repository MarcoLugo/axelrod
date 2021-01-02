use crate::player::Choice;

pub fn cooperator(_: &[Choice], _: &[Choice]) -> Choice {
    Choice::Cooperate
}

pub fn defector(_: &[Choice], _: &[Choice]) -> Choice {
    Choice::Defect
}

pub fn alternator_cooperator(my_choices: &[Choice], _: &[Choice]) -> Choice {
    match my_choices.last() {
        Some(Choice::Cooperate) => Choice::Defect,
        Some(Choice::Defect) => Choice::Cooperate,
        None => Choice::Cooperate,
    }
}

pub fn alternator_defector(my_choices: &[Choice], _: &[Choice]) -> Choice {
    match my_choices.last() {
        Some(Choice::Cooperate) => Choice::Defect,
        Some(Choice::Defect) => Choice::Cooperate,
        None => Choice::Defect,
    }
}

pub fn tit_for_tat(_: &[Choice], other_choices: &[Choice]) -> Choice {
    match other_choices.last() {
        Some(Choice::Defect) => Choice::Defect,
        _ => Choice::Cooperate,
    }
}

pub fn random_coin_flip(my_choices: &[Choice], _: &[Choice]) -> Choice {
    // TODO: make this strategy independent from other strategies using RNG.
    if my_choices.is_empty() {
        fastrand::seed(404);
    }

    if fastrand::bool() {
        Choice::Cooperate
    } else {
        Choice::Defect
    }
}
