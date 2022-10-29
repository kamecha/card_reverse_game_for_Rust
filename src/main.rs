use std::io::{stdout, Write};
use crossterm::{
	execute,
	Result, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, cursor::{Hide, Show},
};

fn main() -> Result<()> {
	enable_raw_mode()?;
	execute!(stdout(), Hide, EnterAlternateScreen)?;

	execute!(stdout(), crossterm::style::Print("Hello world!"))?;

	execute!(stdout(), Show, LeaveAlternateScreen)?;
	disable_raw_mode()?;
	Ok(())
}

