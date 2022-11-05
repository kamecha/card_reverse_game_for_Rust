#[derive(Copy, Clone, PartialEq)]
pub enum CardType {
    Circle,
    Square,
    Triangle,
    Cross,
    Empty,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Card {
    pub card_type: CardType,
    pub is_reverse: bool,
}

pub struct Model {
    // model上はカードの状態のみを管理する
    pub cards: Vec<Card>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            cards: vec![
                Card {
                    card_type: CardType::Circle,
                    is_reverse: true,
                };
                8
            ],
        }
    }
    pub fn get_card(&self, index: usize) -> Card {
        self.cards[index]
    }
    pub fn set_card(&mut self, index: usize, card: Card) {
        self.cards[index] = card;
    }
    pub fn reverse_card(&mut self, index: usize) {
        self.cards[index].is_reverse = !self.cards[index].is_reverse;
    }
}
