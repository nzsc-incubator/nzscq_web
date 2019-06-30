use nzscq::{
    choices::{Action, BatchChoices, Booster, Character, DequeueChoice},
    game::BatchChoiceGame,
};

pub struct Opponent<T: Random> {
    prng: T,
}

impl<T: std::fmt::Debug + Random> std::fmt::Debug for Opponent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Opponent {{ prng: {:?} }}", self.prng)
    }
}

impl<T: Clone + Random> Clone for Opponent<T> {
    fn clone(&self) -> Opponent<T> {
        Opponent {
            prng: self.prng.clone(),
        }
    }
}

impl<T: Random> Opponent<T> {
    const COMPUTER: usize = 1;

    pub fn new(prng: T) -> Opponent<T> {
        Opponent { prng }
    }

    pub fn choose_character(&mut self, game: &BatchChoiceGame) -> Option<Character> {
        if let BatchChoices::Characters(mut choices) = game.choices() {
            let computer_choices = choices.remove(Opponent::<T>::COMPUTER);
            Some(self.rand_choice(computer_choices))
        } else {
            None
        }
    }

    pub fn choose_booster(&mut self, game: &BatchChoiceGame) -> Option<Booster> {
        if let BatchChoices::Boosters(mut choices) = game.choices() {
            let computer_choices: Vec<Booster> = choices
                .remove(Opponent::<T>::COMPUTER)
                .into_iter()
                .filter(|&booster| booster != Booster::None)
                .collect();
            Some(self.rand_choice(computer_choices))
        } else {
            None
        }
    }

    pub fn choose_dequeue(&mut self, game: &BatchChoiceGame) -> Option<DequeueChoice> {
        if let BatchChoices::DequeueChoices(mut choices) = game.choices() {
            let computer_choices = prefer_drain_and_exit(choices.remove(Opponent::<T>::COMPUTER));
            Some(self.rand_choice(computer_choices))
        } else {
            None
        }
    }

    pub fn choose_action(&mut self, game: &BatchChoiceGame) -> Option<Action> {
        if let BatchChoices::Actions(mut choices) = game.choices() {
            let computer_choices = choices.remove(Opponent::<T>::COMPUTER);
            Some(self.rand_choice(computer_choices))
        } else {
            None
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

pub trait Random {
    fn random(&mut self) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Difficulty {
    Stupid,
    Easy,
    Medium,
}