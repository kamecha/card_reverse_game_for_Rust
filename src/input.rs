use super::*;
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    Result,
};

pub fn input(
    event: Event,
    cursor: &mut (u16, u16),
    _cards: &mut Vec<Vec<Card>>,
    width: u16,
    height: u16,
    end: &mut bool,
) -> Result<()> {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
        }) => *end = true,
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
        _ => {}
    }
    return Ok(());
}
