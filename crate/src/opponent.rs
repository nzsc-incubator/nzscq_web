use nzscq::{
    choices::{BatchChoices, Booster, Character},
    game::BatchChoiceGame,
};

pub struct Opponent<T: Random> {
    prng: T,
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
            let computer_choices = choices.remove(Opponent::<T>::COMPUTER);
            Some(self.rand_choice(computer_choices))
        } else {
            None
        }
    }

    fn rand_choice<C>(&mut self, mut choices: Vec<C>) -> C {
        let len = choices.len() as f64;
        let index = len * self.prng.random();

        choices.remove(0 as usize)
    }
}

pub trait Random {
    fn random(&mut self) -> f64;
}
