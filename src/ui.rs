use ratatui::{
    Terminal,
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    text::Text,
    widgets::{Block, BorderType, Borders},
};

use crate::{TormResult, state::State};

/// Draws ui.
pub async fn draw(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &mut State,
) -> TormResult<()> {
    let buff = &state.test_buffer;

    terminal.draw(|frame| {
        let chunks =
            Layout::horizontal([Constraint::Length(5), Constraint::Length(1), Constraint::Min(0)])
                .split(frame.area());

        // let num_chunks = Layout::vertical(
        //     vec![Constraint::Length(1); buff.line_count]
        // ).split(chunks[0]);
        //
        for i in 0..buff.line_count() {
            frame.render_widget(
                Text::from((i + 1).to_string()).right_aligned(),
                Rect::new(0, i as u16, chunks[0].width, 1),
            );
        }

        frame.render_widget(
            Block::new()
                .borders(Borders::RIGHT)
                .border_type(BorderType::Rounded),
            chunks[1],
        );

        frame.render_widget(Text::from(buff.content().as_str()), chunks[2]);
    })?;

    Ok(())
}
