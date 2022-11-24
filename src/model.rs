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
    // true: マークが見えない、false: マークが見える
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
                },
                Card {
                    card_type: CardType::Circle,
                    is_reverse: true,
                },
                Card {
                    card_type: CardType::Square,
                    is_reverse: true,
                },
                Card {
                    card_type: CardType::Square,
                    is_reverse: true,
                },
                Card {
                    card_type: CardType::Triangle,
                    is_reverse: true,
                },
                Card {
                    card_type: CardType::Triangle,
                    is_reverse: true,
                },
                Card {
                    card_type: CardType::Cross,
                    is_reverse: true,
                },
                Card {
                    card_type: CardType::Cross,
                    is_reverse: true,
                },
            ],
        }
    }
    pub fn get_card(&self, index: usize) -> Card {
        self.cards[index]
    }
    pub fn set_card(&mut self, index: usize, card: Card) {
        self.cards[index] = card;
    }
    pub fn number_of_reverse_card(&self) -> usize {
        let mut count = 0;
        for card in &self.cards {
            if card.is_reverse {
                count += 1;
            }
        }
        count
    }
    pub fn number_of_not_reverse_card(&self) -> usize {
        self.cards.len() - self.number_of_reverse_card()
    }
    pub fn reverse_card(&mut self, index: usize) {
        self.cards[index].is_reverse = !self.cards[index].is_reverse;
    }
}
