use std::panic;

use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{
    TormResult,
    event::{EventHandler, TormEvent},
    state::State,
};

/// Torm Code Editor.
pub struct Torm {
    /// Ratatui terminal.
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,

    /// Event handler.
    event_handler: EventHandler,

    /// Torm state.
    state: State,
}

impl Torm {
    pub fn new() -> TormResult<Self> {
        // Hook into panics to properly restore the terminal
        // when a panic happened.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            let _ = Torm::restore();

            panic_hook(panic);
        }));

        let terminal = ratatui::init();
        let event_handler = EventHandler::new(60);
        let state = State::new();

        Ok(Self {
            terminal,
            event_handler,
            state,
        })
    }

    /// Start torm.
    pub async fn start(&mut self) -> TormResult<()> {
        // Start event handler.
        self.event_handler.handle();

        // Run indefinitely.
        while !self.state.exit {
            match self.event_handler.next().await? {
                TormEvent::Tick => self.tick().await?,
                TormEvent::KeyPress(key) => self.keypress(key).await?,
                TormEvent::MouseClick(mouse, btn) => self.mouse_click(mouse, btn).await?,
            }
        }

        Ok(())
    }

    /// Handle tick event.
    async fn tick(&mut self) -> TormResult<()> {
        // Draws the ui.
        // ui::draw(&mut self.terminal, &self.state).await?;

        Ok(())
    }

    /// Handle keypress event.
    async fn keypress(&mut self, key: crossterm::event::KeyEvent) -> TormResult<()> {
        match key.code {
            crossterm::event::KeyCode::Esc => self.exit()?,

            _ => {}
        }

        Ok(())
    }

    /// Handle mouse click event.
    async fn mouse_click(
        &mut self,
        _mouse: crossterm::event::MouseEvent,
        _button: crossterm::event::MouseButton,
    ) -> TormResult<()> {
        Ok(())
    }

    /// Restore terminal state.
    fn restore() -> TormResult<()> {
        // Restore terminal.
        ratatui::restore();

        // Disables mouse capture.
        crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;

        Ok(())
    }

    /// Exits out of fum.
    fn exit(&mut self) -> TormResult<()> {
        Torm::restore()?;

        self.state.exit = true;

        Ok(())
    }
}
