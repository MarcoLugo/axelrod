# axelrod 

*axelrod* is an iterated prisoner's dilemma (IPD) tournament implementation inspired by Robert Axelrod's great book, The Evolution of Cooperation. It is fast and meant to be as simple as possible for those who want to contribute new strategies or use it for their own projects.

---

## Strategies

### Strategy Model

Strategies are defined in the *src/strategies.rs* file. They are represented as a single function whose signature is given by *StrategySignature*, defined in *src/tournament.rs*.

The function signature must be:

```rust
Fn(&[Choice], &[Choice], &mut Rng) -> Choice
```

The first slice represents the history of the player's own choices, the second slice represents the history of the opponent's choices and, finally, the *Rng* object is a mutable reference to the player's random number generator. Each strategy can thus use the full history of interactions as well as a random number. The function must return one of the two possibles choices as defined by the *Choice* enum (i.e., Cooperate or Defect). Two notable examples below:

#### Tit For Tat

```rust
pub fn tit_for_tat(_: &[Choice], other_choices: &[Choice], _: &mut Rng) -> Choice {
    match other_choices.last() {
        None => Choice::Cooperate,
        Some(&c) => c,
    }
}
```

This implementation of Anatol Rapoport's infamous tit for tat does several things:
- It ignores the player's own choices (this is done by using the underscore instead of assigning a name to the first parameter).
- It takes the opponent's choices (*other_choices*) and pattern matches on it: if it is empty then it is the first move and therefore it returns *Choice::Cooperate*, otherwise it returns whatever was the last move of the oponent.
- As with the player's own history, it ignores the random number generator by using an underscore.

#### RANDOM

```rust
pub fn random_coin_flip(_: &[Choice], _: &[Choice], rng: &mut Rng) -> Choice {
    if rng.bool() {
        Choice::Cooperate
    } else {
        Choice::Defect
    }
}
```

This implementation is quite simple. It ignores both players' histories and uses the random number generator method *bool* to obtain a random boolean to make the choice.

### Adding a Strategy to a Tournament

Each strategy must put inside a tuple that contains two elements:
1. A string literal with the name of the player.
2. The strategy implementation (the function described above), [boxed](https://doc.rust-lang.org/std/boxed/struct.Box.html).

A vector of these tuples must be created and then passed to the Tournament constructor.

For example:

```rust
let configs: Vec<(&str, StrategySignature)> = vec![
        ("TitForTat", Box::new(strategies::tit_for_tat)),
        ("RandomCoinFlip", Box::new(strategies::random_coin_flip)),
    ];

let mut tournament = Tournament::new(configs, 100);
```

See *src/main.rs* for more information.

It is important to note that the name of the player is used as a seed passed to the player's random number generator and thus the reproducibility of the player's choices is tied to the player's name.

You are welcome to contribute new strategies either with code or with ideas.

### Currently Implemented Stategies

- Tit For Tat
- Random
- Grim Trigger
- Tit for Two Tats
- Cooperator (always cooperates)
- Defector (always defects)

## Usage

The tournament is defined in *src/main.rs*. Once the desired tournament is defined (player names/strategies being chosen and the number of iterations) all that is left to do is to compile and run normally. The program can be compiled using:

```sh
cargo build --release
```

And then run normally (via a command line or terminal), or built and run directly by using:

```sh
cargo run --release
```

### Example Output

The output below is the result of a 3 player (tit for tat, defector, random), 200 iteration game.

```none
[+] Running tournament...
  [00:00:00] [████████████████████████████████████████] (200/200, ETA 0s)

[+] Round-robin scores:
Id 1 |                  Player 1 | Score 1 | Score 2 | Player 2                  | Id 2
   0 |                  Defector |     200 | 200     | Defector                  | 0   
   0 |                  Defector |     204 | 199     | TitForTat                 | 1   
   0 |                  Defector |     604 | 99      | RandomCoinFlip            | 2   
   1 |                 TitForTat |     600 | 600     | TitForTat                 | 1   
   1 |                 TitForTat |     459 | 464     | RandomCoinFlip            | 2   
   2 |            RandomCoinFlip |     473 | 423     | RandomCoinFlip            | 2   

[+] Ranked total scores:
   id |                      name |      total |    average
    1 |                 TitForTat |       1858 |       9.29
    2 |            RandomCoinFlip |       1459 |      7.295
    0 |                  Defector |       1208 |       6.04
```

## Future Work

* Add test cases.
* Add more documentation.
* Add ability to configure tournament using external configuration file.
* Add more options for tournament result output.
* Add an optional noise feature (to represent miscommunication or misunderstanding between players).
* Add more strategies from the IPD literature.
* Add an evolutionary tournament mode where only the best players (strategies) survive and make it to the next rounds.
* Add an optional mutation component to the evolutionary approach.
* Add a graphical user interface (GUI)

## License

Licensed under the <a href="LICENSE-MIT">MIT license</a>.
