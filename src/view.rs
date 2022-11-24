use super::*;
use crossterm::Result;

pub fn view<T: std::io::Write>(
    output: &mut T,
    game: &gameStract::GameStract,
    width: u16,
    height: u16,
    cursor: &(u16, u16),
) -> Result<()> {
    let base = (3, 3);
    let card_size = 3;
    let card_spacing = 2;
    execute!(
        output,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )?;
    for i in 0..height {
        for j in 0..width {
            let mut mark = String::new();
            let card = game.model.get_card((i * width + j) as usize);
            if card.is_reverse {
                mark.push(' ');
            } else {
                match card.card_type {
                    model::CardType::Circle => mark.push('o'),
                    model::CardType::Square => mark.push('□'),
                    model::CardType::Triangle => mark.push('^'),
                    model::CardType::Cross => mark.push('x'),
                    model::CardType::Empty => continue,
                }
            }
            // cardの描画
            let col = base.0 + j * (card_size + card_spacing);
            let row = base.1 + i * (card_size + card_spacing);
            if game.model.get_card((i * width + j) as usize).card_type != model::CardType::Empty {
                execute!(output, MoveTo(col, row),)?;
                let tmp = ["|", "|"].join(&mark);
                execute!(output, Print("+-+\n"))?;
                execute!(output, MoveTo(col, row + 1),)?;
                execute!(output, Print(tmp))?;
                execute!(output, MoveTo(col, row + 2),)?;
                execute!(output, Print("+-+\n"))?;
            }
            if *cursor == (i, j) {
                // cursorの描画
                let cursor_base = (base.0 - 1, base.1 - 1);
                let cursor_size = card_size + 2;
                let cursor_col = cursor_base.0 + cursor.1 * cursor_size;
                let cursor_row = cursor_base.1 + cursor.0 * cursor_size;
                execute!(output, MoveTo(cursor_col, cursor_row),)?;
                execute!(output, Print("+---+\n"))?;
                execute!(output, MoveTo(cursor_col, cursor_row + 1),)?;
                execute!(output, Print("|\n"))?;
                execute!(output, MoveTo(cursor_col + 4, cursor_row + 1),)?;
                execute!(output, Print("|\n"))?;
                execute!(output, MoveTo(cursor_col, cursor_row + 2),)?;
                execute!(output, Print("|\n"))?;
                execute!(output, MoveTo(cursor_col + 4, cursor_row + 2),)?;
                execute!(output, Print("|\n"))?;
                execute!(output, MoveTo(cursor_col, cursor_row + 3),)?;
                execute!(output, Print("|\n"))?;
                execute!(output, MoveTo(cursor_col + 4, cursor_row + 3),)?;
                execute!(output, Print("|\n"))?;
                execute!(output, MoveTo(cursor_col, cursor_row + 4),)?;
                execute!(output, Print("+---+\n"))?;
            }
        }
        // execute!(std::io::stdout(), Print("\n"))?;
    }
    return Ok(());
}
