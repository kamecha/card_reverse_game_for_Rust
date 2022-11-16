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
    pub fn reverse_card(&mut self, index: usize) {
        // Playerの確認
        // 一度カードを反転すると、もう一度反転できない
        if self.model.get_card(index).is_reverse {
            return;
        }
        // ペアの確認
        // 最終的にカードを反転する
        self.model.reverse_card(index);
    }
}
