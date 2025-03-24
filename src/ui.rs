use ratatui::{Terminal, prelude::CrosstermBackend, text::Text};

use crate::TormResult;

/// Draws ui.
pub async fn draw(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> TormResult<()> {
    terminal.draw(|frame| {
        frame.render_widget(Text::from("torm foo bar"), frame.area());
    })?;

    Ok(())
}
