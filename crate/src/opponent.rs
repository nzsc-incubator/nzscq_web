use nzscq::{
    choices::{Action, BatchChoices, Booster, Character, DequeueChoice, PointsAgainst},
    game::BatchChoiceGame,
};

use std::convert::TryFrom;
use std::fmt::{self, Debug, Display, Formatter};

pub struct Opponent {
    difficulty: Difficulty,
    prng: Box<dyn Random>,
}

impl Debug for Opponent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Opponent {{ difficulty: {:?}, prng: {:?} }}",
            self.difficulty, self.prng,
        )
    }
}

impl Opponent {
    const COMPUTER: usize = 1;

    pub fn new(difficulty: Difficulty, prng: Box<dyn Random>) -> Opponent {
        Opponent { difficulty, prng }
    }

    pub fn choose_character(&mut self, game: &BatchChoiceGame) -> Option<Character> {
        if let BatchChoices::Characters(mut choices) = game.choices() {
            let computer_choices = choices.remove(Opponent::COMPUTER);
            Some(self.rand_choice(computer_choices))
        } else {
            None
        }
    }

    pub fn choose_booster(&mut self, game: &BatchChoiceGame) -> Option<Booster> {
        if let BatchChoices::Boosters(mut choices) = game.choices() {
            let computer_choices = choices.remove(Opponent::COMPUTER);
            let computer_choices: Vec<Booster> = match self.difficulty {
                Difficulty::Stupid => computer_choices,
                _ => computer_choices
                    .into_iter()
                    .filter(|&booster| booster != Booster::None)
                    .collect(),
            };

            Some(self.rand_choice(computer_choices))
        } else {
            None
        }
    }

    pub fn choose_dequeue(&mut self, game: &BatchChoiceGame) -> Option<DequeueChoice> {
        if let BatchChoices::DequeueChoices(mut choices) = game.choices() {
            let computer_choices = choices.remove(Opponent::COMPUTER);
            let computer_choices = match self.difficulty {
                Difficulty::Stupid => computer_choices,
                // TODO Make medium drain more cautiously
                _ => prefer_drain_and_exit(computer_choices),
            };

            Some(self.rand_choice(computer_choices))
        } else {
            None
        }
    }

    pub fn choose_action(&mut self, game: &BatchChoiceGame) -> Option<Action> {
        match self.difficulty {
            Difficulty::Stupid | Difficulty::Easy => {
                if let BatchChoices::Actions(mut choices) = game.choices() {
                    let computer_choices = choices.remove(Opponent::COMPUTER);
                    Some(self.rand_choice(computer_choices))
                } else {
                    None
                }
            }
            Difficulty::Medium => actions_that_make_sense_for_medium_difficulty_computer(game)
                .map(|actions| self.rand_choice(actions)),
        }

    }

    fn rand_choice<C>(&mut self, mut choices: Vec<C>) -> C {
        let len = choices.len() as f64;
        let index = len * self.prng.random();

        choices.remove(index as usize)
    }
}

fn prefer_drain_and_exit(choices: Vec<DequeueChoice>) -> Vec<DequeueChoice> {
    if choices.iter().any(is_drain_and_exit) {
        choices.into_iter().filter(is_drain_and_exit).collect()
    } else if choices.iter().any(is_just_exit) {
        choices.into_iter().filter(is_just_exit).collect()
    } else {
        choices
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_drain_and_exit(choice: &DequeueChoice) -> bool {
    if let DequeueChoice::DrainAndExit(_) = choice {
        true
    } else {
        false
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_just_exit(&choice: &DequeueChoice) -> bool {
    choice == DequeueChoice::JustExit
}

fn actions_that_make_sense_for_medium_difficulty_computer(
    game: &BatchChoiceGame,
) -> Option<Vec<Action>> {
    if let BatchChoices::Actions(mut choices) = game.choices() {
        let human_choices = choices.remove(0);
        let computer_choices = choices.remove(0);
        let mut scoreboard = game
            .scoreboard()
            .actionless()
            .expect("should be on action-choosing phase");
        let human = scoreboard.remove(0);
        let guarantees_win: Box<dyn Fn(&Action) -> bool> = Box::new(|&action| {
            human_choices
                .iter()
                .all(|&human_action| PointsAgainst::points_of(&[action, human_action]) == [1, 0])
        });
        let guarantees_point: Box<dyn Fn(&Action) -> bool> = Box::new(|&action| {
            human_choices
                .iter()
                .all(|&human_action| PointsAgainst::points_of(&[action, human_action])[0] == 1)
        });
        let guarantees_human_wont_get_point: Box<dyn Fn(&Action) -> bool> = Box::new(|&action| {
            human_choices
                .iter()
                .all(|&human_action| PointsAgainst::points_of(&[action, human_action])[1] == 0)
        });
        let win_possible: Box<dyn Fn(&Action) -> bool> = Box::new(|&action| {
            human_choices
                .iter()
                .any(|&human_action| PointsAgainst::points_of(&[action, human_action]) == [1, 0])
        });
        let non_loss_possible: Box<dyn Fn(&Action) -> bool> = Box::new(|&action| {
            human_choices
                .iter()
                .any(|&human_action| PointsAgainst::points_of(&[action, human_action]) != [0, 1])
        });

        if human.points == 4 {
            Some(prefer(
                computer_choices,
                vec![
                    guarantees_win,
                    guarantees_human_wont_get_point,
                    win_possible,
                    non_loss_possible,
                ],
            ))
        } else {
            Some(prefer(
                computer_choices,
                vec![
                    guarantees_win,
                    guarantees_point,
                    win_possible,
                    non_loss_possible,
                ],
            ))
        }
    } else {
        None
    }
}

fn prefer<'a, T>(choices: Vec<T>, predicates: Vec<Box<dyn (Fn(&T) -> bool) + 'a>>) -> Vec<T>
where
    T: Clone,
{
    for p in predicates {
        let satisfactory_choices: Vec<T> = choices.iter().cloned().filter(|i| p(i)).collect();
        if !satisfactory_choices.is_empty() {
            return satisfactory_choices;
        }
    }

    choices
}

pub trait Random: Debug {
    fn random(&mut self) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Difficulty {
    Stupid = 0,
    Easy = 1,
    Medium = 2,
}

impl TryFrom<u8> for Difficulty {
    type Error = ();

    fn try_from(x: u8) -> Result<Difficulty, ()> {
        match x {
            0 => Ok(Difficulty::Stupid),
            1 => Ok(Difficulty::Easy),
            2 => Ok(Difficulty::Medium),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Difficulty {
    type Error = ();

    fn try_from(x: &str) -> Result<Difficulty, ()> {
        match &x.to_ascii_lowercase()[..] {
            "stupid" => Ok(Difficulty::Stupid),
            "easy" => Ok(Difficulty::Easy),
            "medium" => Ok(Difficulty::Medium),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for Difficulty {
    type Error = ();

    fn try_from(x: String) -> Result<Difficulty, ()> {
        Difficulty::try_from(&x[..])
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Difficulty::Stupid => write!(f, "Stupid"),
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Medium => write!(f, "Medium"),
        }
    }
}