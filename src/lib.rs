pub mod render {
    use crate::game::{Game, Card, CardType};
	#[allow(dead_code)]
    pub struct Grid {
        base: (i32, i32),
        height: i32,
        width: i32,
        card_size: i32,
        padding: i32,
    }
    pub struct Point {
        pub x_index: i32,
        pub y_index: i32,
    }
    impl Grid {
        pub fn new(
            base: (i32, i32),
            height: i32,
            width: i32,
            card_size: i32,
            padding: i32,
        ) -> Grid {
            Grid {
                base: base,
                height: height,
                width: width,
                card_size: card_size,
                padding: padding,
            }
        }
        pub fn get_card_pos(&self, x_index: i32, y_index: i32) -> (i32, i32) {
            let x = self.base.0 + (x_index * (self.card_size + self.padding));
            let y = self.base.1 + (y_index * (self.card_size + self.padding));
            (x, y)
        }
        pub fn get_card_size(&self) -> i32 {
            self.card_size
        }
    }
    pub fn render_card(grid: &Grid, pont: &Point, card: &Card) {
        let card_type: String;
        match card.card_type {
            CardType::Circle => {
                card_type = "○".to_string();
            }
            CardType::Square => {
                card_type = "■".to_string();
            }
            CardType::Triangle => {
                card_type = "▲".to_string();
            }
            CardType::Cross => {
                card_type = "✕".to_string();
            }
        }
        let (x, y) = grid.get_card_pos(pont.x_index, pont.y_index);
        println!("\x1B[{};{}H{}", y, x, "┌─┐");
        println!(
            "\x1B[{};{}H|{}|",
            y + 1,
            x,
            if card.is_reverse { " " } else { &card_type }
        );
        println!("\x1B[{};{}H{}", y + 2, x, "└─┘");
    }
	pub fn render(game: &Game, grid: &Grid) {
		let mut point_index = Point { x_index: 0, y_index: 0 };
		for card in &game.deck {
			render_card(grid, &point_index, &card);
			point_index.x_index += 1;
		}
	}
	pub fn cursor_render(grid: &Grid, point: &Point) {
		let (x, y) = grid.get_card_pos(point.x_index, point.y_index);
		x = if x > grid.width { grid.width } else { x };
		x = if x < 0 { 0 } else { x };
		y = if y > grid.height { grid.height } else { y };
		y = if y < 0 { 0 } else { y };
		println!("\x1B[{};{}H{}", y - 1, x - 1, "+---+");
		println!("\x1B[{};{}H{}", y    , x - 1, "|");
		println!("\x1B[{};{}H{}", y    , x + 3, "|");
		println!("\x1B[{};{}H{}", y + 1, x - 1, "|");
		println!("\x1B[{};{}H{}", y + 1, x + 3, "|");
		println!("\x1B[{};{}H{}", y + 2, x - 1, "|");
		println!("\x1B[{};{}H{}", y + 2, x + 3, "|");
		println!("\x1B[{};{}H{}", y + 3, x - 1, "+---+");
	}
	pub fn cursor_render_not(grid: &Grid, point: &Point) {
		let (x, y) = grid.get_card_pos(point.x_index, point.y_index);
		println!("\x1B[{};{}H{}", y - 1, x - 1, "     ");
		println!("\x1B[{};{}H{}", y    , x - 1, " ");
		println!("\x1B[{};{}H{}", y    , x + 3, " ");
		println!("\x1B[{};{}H{}", y + 1, x - 1, " ");
		println!("\x1B[{};{}H{}", y + 1, x + 3, " ");
		println!("\x1B[{};{}H{}", y + 2, x - 1, " ");
		println!("\x1B[{};{}H{}", y + 2, x + 3, " ");
		println!("\x1B[{};{}H{}", y + 3, x - 1, "     ");
	}
}

pub mod input {
	use crate::render::{Grid, Point};
	pub struct Cursor {
		pub x_index: i32,
		pub y_index: i32,
	}
	impl Cursor {
		pub fn new() -> Cursor {
			Cursor {
				x_index: 0,
				y_index: 0,
			}
		}
		pub fn move_right(&mut self) {
			self.x_index += 1;
			self.x_index = if self.x_index > Grid.width { Grid.width } else { self.x_index };
		}
		pub fn move_left(&mut self) {
			self.x_index -= 1;
			self.x_index = if self.x_index < 0 { 0 } else { self.x_index };
		}
		pub fn move_up(&mut self) {
			self.y_index -= 1;
			self.y_index = if self.y_index < 0 { 0 } else { self.y_index };
		}
		pub fn move_down(&mut self) {
			self.y_index += 1;
			self.y_index = if self.y_index > Grid.height { Grid.height } else { self.y_index };
		}
	}
}

pub mod game {
    pub enum CardType {
        Circle,
        Square,
        Triangle,
        Cross,
    }
    pub struct Card {
        pub card_type: CardType,
        pub is_reverse: bool,
    }
    pub struct Game {
        pub deck: Vec<Card>,
        // players: Vec<Player>,
        pub turn: i32,
    }
    impl Game {
        pub fn game_start(&mut self) {
            println!("Game Start!");
            self.turn = 0;
        }
    }
}
