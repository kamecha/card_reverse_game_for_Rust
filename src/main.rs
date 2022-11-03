mod input;
mod view;
use crossterm::{
    cursor::{MoveTo, Show},
    event::read,
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

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
    card_type: CardType,
    is_reverse: bool,
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stdout(), Show, EnterAlternateScreen)?;

    const HEIGHT: u16 = 2;
    const WIDTH: u16 = 4;
    let mut cards = vec![
        vec![
            Card {
                card_type: CardType::Empty,
                is_reverse: false,
            };
            WIDTH as usize
        ];
        HEIGHT as usize
    ];
    let mut cursor = (0, 0);
    let mut end = false;
    while !end {
        // view
        view::view(&mut std::io::stdout(), &cards, &cursor)?;
        // input
        input::input(read()?, &mut cursor, &mut cards, WIDTH, HEIGHT, &mut end)?;
    }

    execute!(std::io::stdout(), Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
