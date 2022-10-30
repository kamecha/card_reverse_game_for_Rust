use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

#[derive(Copy, Clone, PartialEq)]
enum CardType {
    Circle,
    Square,
    Triangle,
    Cross,
    Empty,
}

#[derive(Copy, Clone, PartialEq)]
struct Card {
    card_type: CardType,
    is_reverse: bool,
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stdout(), Show, EnterAlternateScreen)?;

    const height: u16 = 2;
    const width: u16 = 4;
    let mut cards = [[Card {
        card_type: CardType::Empty,
        is_reverse: false,
    }; width as usize]; height as usize];
    let mut cursor = (0, 0);
    loop {
        // view
        let base = (3, 3);
        let card_size = 3;
        let card_spacing = 2;
        // clear the terminal
        execute!(
            std::io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )?;
        for i in 0..2 {
            for j in 0..4 {
                let hoge = "o";
                if cursor == (i, j) {
                    // cursorの描画
                    let cursor_base = (base.0 - 1, base.1 - 1);
                    let cursor_size = card_size + 2;
                    let cursor_col = cursor_base.0 + cursor.1 * cursor_size;
                    let cursor_row = cursor_base.1 + cursor.0 * cursor_size;
                    let tmp = ["||", "||"].connect(hoge);
                    execute!(std::io::stdout(), MoveTo(cursor_col, cursor_row),)?;
                    execute!(std::io::stdout(), Print("+---+\n"))?;
                    execute!(std::io::stdout(), MoveTo(cursor_col, cursor_row + 1),)?;
                    execute!(std::io::stdout(), Print("|+-+|\n"))?;
                    execute!(std::io::stdout(), MoveTo(cursor_col, cursor_row + 2),)?;
                    execute!(std::io::stdout(), Print(tmp))?;
                    execute!(std::io::stdout(), MoveTo(cursor_col, cursor_row + 3),)?;
                    execute!(std::io::stdout(), Print("|+-+|\n"))?;
                    execute!(std::io::stdout(), MoveTo(cursor_col, cursor_row + 4),)?;
                    execute!(std::io::stdout(), Print("+---+\n"))?;
                } else {
                    // cardの描画
                    let col = base.0 + j * (card_size + card_spacing);
                    let row = base.1 + i * (card_size + card_spacing);
                    execute!(std::io::stdout(), MoveTo(col, row),)?;
                    match cards[i as usize][j as usize].card_type {
                        CardType::Empty => {
                            let tmp = ["|", "|"].connect(hoge);
                            execute!(std::io::stdout(), Print("+-+\n"))?;
                            execute!(std::io::stdout(), MoveTo(col, row + 1),)?;
                            execute!(std::io::stdout(), Print(tmp))?;
                            execute!(std::io::stdout(), MoveTo(col, row + 2),)?;
                            execute!(std::io::stdout(), Print("+-+\n"))?;
                        }
                        _ => {}
                    }
                }
            }
            // execute!(std::io::stdout(), Print("\n"))?;
        }
        // input
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                if cursor.1 > 0 {
                    cursor.1 -= 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                if cursor.1 < width - 1 {
                    cursor.1 += 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                if cursor.0 > 0 {
                    cursor.0 -= 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                if cursor.0 < height - 1 {
                    cursor.0 += 1;
                }
            }
            _ => continue,
        }
    }

    execute!(std::io::stdout(), Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
