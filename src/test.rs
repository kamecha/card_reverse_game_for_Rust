#[cfg(test)]
mod tests {
    use crate::input::input;
    use crate::{Card, CardType};
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
    #[test]
    fn input_test() {
        let mut cards = vec![
            vec![
                Card {
                    card_type: CardType::Empty,
                    is_reverse: false,
                };
                4
            ];
            2
        ];
        let mut cursor = (0, 0);
        let mut end = false;
        let left_key = Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
        });
        let right_key = Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
        });
        let down_key = Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
        });
        let up_key = Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
        });
        input(down_key, &mut cursor, &mut cards, 4, 2, &mut end).unwrap();
        assert!(cursor == (1, 0));
        input(right_key, &mut cursor, &mut cards, 4, 2, &mut end).unwrap();
        assert!(cursor == (1, 1));
        input(up_key, &mut cursor, &mut cards, 4, 2, &mut end).unwrap();
        assert!(cursor == (0, 1));
        input(left_key, &mut cursor, &mut cards, 4, 2, &mut end).unwrap();
        assert!(cursor == (0, 0));
        input(up_key, &mut cursor, &mut cards, 4, 2, &mut end).unwrap();
        assert!(cursor == (0, 0));
        input(left_key, &mut cursor, &mut cards, 4, 2, &mut end).unwrap();
        assert!(cursor == (0, 0));
    }

    #[test]
    fn view_test() {}
}
