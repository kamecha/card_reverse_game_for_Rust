mod input;
mod model;
mod test;
mod view;
use crossterm::{
    cursor::{MoveTo, Show},
    event::read,
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stdout(), Show, EnterAlternateScreen)?;

    const HEIGHT: u16 = 2;
    const WIDTH: u16 = 4;
    let mut model = model::Model::new();
    let mut cursor = (0, 0);
    let mut end = false;
    while !end {
        // view
        view::view(&mut std::io::stdout(), &model, WIDTH, HEIGHT, &cursor)?;
        // input
        input::input(read()?, &mut cursor, &mut model, WIDTH, HEIGHT, &mut end)?;
    }

    execute!(std::io::stdout(), Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
