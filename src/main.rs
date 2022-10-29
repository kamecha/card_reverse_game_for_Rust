use card_reverse_game::game::{Card, CardType, Game};
use card_reverse_game::render::{Grid, self};
use card_reverse_game::input;
use std::io::{stdin, Cursor};
use std::thread::{sleep, spawn};
use std::time::{Duration, SystemTime};

fn game_loop(game: &Game, render: &Grid) {
    let mut time = SystemTime::now();
	let delta_taime = Duration::from_secs_f64(1.0 / 60.0);
	let point = render::Point {
		x_index: 0,
		y_index: 0,
	};
    loop {
		render::render(game, render);
		// render::cursor_render_not(render, &point);
		render::cursor_render(render, &point);
        time += delta_taime;
        if let Ok(dur) = time.duration_since(SystemTime::now()) {
            sleep(dur);
		}
    }
}

fn main() {
    // 画面の初期化
    println!("\x1B[2J");
    let game = Game {
        deck: vec![
			Card {
				card_type: CardType::Circle,
				is_reverse: false,
			},
			Card {
				card_type: CardType::Triangle,
				is_reverse: false,
			},
		], // 山札
        turn: 0, // ターン数
    };
    let grid = Grid::new((3, 3), 3, 3, 3, 2);
	let mut cursor = input::Cursor::new();
	spawn(|| {
		let input = stdin();
		let mut buf = String::new();
		loop {
			input.read_line(&mut buf).unwrap();
			match &buf[..] {
				"q\n" => {
					println!("exit");
				}
				// "left\n" => {
				// 	cursor.move_left();
				// }
				// "right\n" => {
				// 	cursor.move_right(&grid);
				// }
				// "up\n" => {
				// 	cursor.move_up();
				// }
				// "down\n" => {
				// 	cursor.move_down(&grid);
				// }
				_ => {
					println!("input error");
				}
			}
			println!("{}", &buf);
			buf.clear();
		}
	});
    game_loop(&game, &grid);
}
