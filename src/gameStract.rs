use super::*;

pub struct Player {
    name: String,
    score: u32,
    cards: Vec<model::Card>,
}

pub struct GameStract {
    pub model: model::Model,
    pub players: Vec<Player>,
    pub turn: i32,
}

impl GameStract {
    const MAX_NUMBER_OF_PAIR: usize = 2;
    pub fn new() -> Self {
        Self {
            model: model::Model::new(),
            players: vec![Player {
                name: "player1".to_string(),
                score: 0,
                cards: vec![],
            }],
            turn: 0,
        }
    }
    pub fn game_start(&mut self) {
        self.turn = 0;
    }
    pub fn turn_start(&mut self) {}
    pub fn turn_end(&mut self) {
        self.turn += 1;
    }
    // 表向いてるカードの種類のペアを判定する
    fn check_card_pair(&mut self) {
        let mut first_card: Option<&mut model::Card> = None;
        for card in &mut self.model.cards {
            if !&card.is_reverse {
                match first_card {
                    Some(_) => {
                        if first_card.as_mut().unwrap().card_type == card.card_type {
                            // ペアが見つかった
                            first_card.as_mut().unwrap().is_reverse = true;
                            card.is_reverse = true;
                            first_card.as_mut().unwrap().card_type = model::CardType::Empty;
                            card.card_type = model::CardType::Empty;
                            self.players[0].score += 1;
                        } else {
                            // ペアが見つからなかった
                            first_card.as_mut().unwrap().is_reverse = true;
                            card.is_reverse = true;
                        }
                    }
                    None => {
                        first_card = Some(card);
                    }
                }
            }
        }
    }
    pub fn reverse_card(&mut self, index: usize) {
        // Playerの確認
        // カードの状態の確認
        if self.model.get_card(index).card_type == model::CardType::Empty {
            return;
        }
        // 一度カードを反転すると、もう一度反転できない
        if !self.model.get_card(index).is_reverse {
            return;
        }
        // 最大で2枚までカードを反転できる
        if self.model.number_of_not_reverse_card() >= Self::MAX_NUMBER_OF_PAIR {
            return;
        }
        self.model.reverse_card(index);
        // ペアの確認
        self.check_card_pair();
    }
}
