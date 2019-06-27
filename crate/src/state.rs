use crate::app::JsPrng;
use crate::helpers;
use crate::opponent::Opponent;
use crate::phase::Phase;

use nzscq::choices::{Action as NzscAction, BatchChoice, Booster, Character, DequeueChoice};
use nzscq::game::BatchChoiceGame;
use nzscq::outcomes::Outcome;
use nzscq::scoreboard::{ActionlessPlayer, DequeueingPlayer};

#[derive(Debug, Clone)]
pub enum State {
    Homescreen,
    SinglePlayer(SinglePlayerState),
}

#[derive(Debug, Clone)]
pub struct SinglePlayerState {
    pub game: BatchChoiceGame,
    pub computer: Opponent<JsPrng>,
    pub phase: Phase,
}

impl SinglePlayerState {
    pub fn handle_character_choice(&mut self, human_character: Character) {
        let previously_available_characters: Vec<Character> = self
            .game
            .choices()
            .characters()
            .expect("should be able to choose character")
            .remove(HUMAN);
        let computer_character = self
            .computer
            .choose_character(&self.game)
            .expect("should choose character");
        let choices = BatchChoice::Characters(vec![human_character, computer_character]);

        let outcome = self.game.choose(choices).expect("should have outcome");

        match outcome {
            Outcome::CharacterPhaseDone(character_headstarts) => {
                self.phase = Phase::ChooseBooster {
                    previously_available_characters,
                    previous_outcome: character_headstarts,
                    available_boosters: self
                        .game
                        .choices()
                        .boosters()
                        .expect("should be able to choose booster")
                        .remove(HUMAN),
                };
            }
            Outcome::CharacterPhaseRechoose(characters) => {
                self.phase = Phase::RechooseCharacter {
                    previously_available_characters,
                    previously_mutually_chosen_character: characters[0],
                    available_characters: self
                        .game
                        .choices()
                        .characters()
                        .expect("should be able to choose character")
                        .remove(HUMAN),
                };
            }
            _ => panic!("outcome should be character outcome"),
        }
    }

    pub fn handle_booster_choice(&mut self, human_booster: Booster) {
        let previously_available_boosters: Vec<Booster> = self
            .game
            .choices()
            .boosters()
            .expect("should be able to choose booster")
            .remove(HUMAN);
        let computer_booster = self
            .computer
            .choose_booster(&self.game)
            .expect("should choose booster");
        let choices = BatchChoice::Boosters(vec![human_booster, computer_booster]);

        let outcome = self.game.choose(choices).expect("should have outcome");

        match outcome {
            Outcome::BoosterPhaseDone(_) => {
                self.phase = Phase::ChooseFirstDequeue {
                    previously_available_boosters,
                    scoreboard: helpers::vec2_to_arr2(
                        self.game
                            .scoreboard()
                            .dequeueing()
                            .expect("should be dequeueing"),
                    ),
                    available_dequeues: helpers::vec2_to_arr2(
                        self.game
                            .choices()
                            .dequeue_choices()
                            .expect("should be able to choose dequeue"),
                    ),
                }
            }
            _ => panic!("outcome should be booster outcome"),
        }
    }

    pub fn handle_dequeue_choice(&mut self, human_dequeue: DequeueChoice) {
        let previous_scoreboard: [DequeueingPlayer; 2] = match &self.phase {
            Phase::ChooseFirstDequeue { scoreboard, .. } => scoreboard.clone(),
            Phase::ChooseSubsequentDequeue { scoreboard, .. } => scoreboard.clone(),
            _ => panic!("should be on a dequeueing phase"),
        };
        let previously_available_dequeues = self
            .game
            .choices()
            .dequeue_choices()
            .expect("should be on a dequeuing phase");

        let computer_dequeue = self
            .computer
            .choose_dequeue(&self.game)
            .expect("should choose dequeue");
        let choices = BatchChoice::DequeueChoices(vec![human_dequeue, computer_dequeue]);
        let outcome = self.game.choose(choices).expect("should have outcome");

        match outcome {
            Outcome::DequeuePhaseDone(dequeues) => {
                self.phase = Phase::ChooseAction {
                    previous_scoreboard,
                    previously_available_dequeues: helpers::vec2_to_arr2(
                        previously_available_dequeues,
                    ),
                    previous_outcome: helpers::vec2_to_arr2(dequeues),
                    scoreboard: helpers::vec2_to_arr2(
                        self.game
                            .scoreboard()
                            .actionless()
                            .expect("should be choosing actions"),
                    ),
                    available_actions: helpers::vec2_to_arr2(
                        self.game
                            .choices()
                            .actions()
                            .expect("should be able to choose action"),
                    ),
                }
            }
            _ => panic!("outcome should be dequeue outcome"),
        }
    }

    pub fn handle_action_choice(&mut self, human_action: NzscAction) {
        let previous_scoreboard: [ActionlessPlayer; 2] = match &self.phase {
            Phase::ChooseAction { scoreboard, .. } => scoreboard.clone(),
            _ => panic!("should be on action-choosing phase"),
        };
        let previously_available_actions = helpers::vec2_to_arr2(
            self.game
                .choices()
                .actions()
                .expect("should be on action-choosing phase"),
        );

        let computer_action = self
            .computer
            .choose_action(&self.game)
            .expect("should choose action");
        let choices = BatchChoice::Actions(vec![human_action, computer_action]);
        let outcome = self.game.choose(choices).expect("should have outcome");

        match outcome {
            Outcome::ActionPhaseDone(action_points_destroyed) => {
                self.phase = Phase::ChooseSubsequentDequeue {
                    previous_scoreboard,
                    previously_available_actions,
                    previous_outcome: helpers::vec2_to_arr2(action_points_destroyed),
                    scoreboard: helpers::vec2_to_arr2(
                        self.game
                            .scoreboard()
                            .dequeueing()
                            .expect("should be dequeueing"),
                    ),
                    available_dequeues: helpers::vec2_to_arr2(
                        self.game
                            .choices()
                            .dequeue_choices()
                            .expect("should be able to choose dequeue"),
                    ),
                }
            }

            Outcome::GameOver(action_points_destroyed) => {
                self.phase = Phase::GameOver {
                    previous_scoreboard,
                    previously_available_actions,
                    previous_outcome: helpers::vec2_to_arr2(action_points_destroyed),
                    scoreboard: helpers::vec2_to_arr2(
                        self.game
                            .scoreboard()
                            .final_()
                            .expect("game should be over"),
                    ),
                }
            }

            _ => panic!("outcome should be action outcome"),
        }
    }

    pub fn restart_game(&mut self) {
        let game = BatchChoiceGame::default();
        let initial_human_choices = game.choices().characters().unwrap().remove(HUMAN);
        let phase = Phase::ChooseCharacter {
            available_characters: initial_human_choices,
        };
        self.game = game;
        self.phase = phase;
    }
}

const HUMAN: usize = 0;