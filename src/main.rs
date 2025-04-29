use torm::Torm;

mod buffer;
mod cursor;
mod event;
mod state;
mod torm;
mod ui;
mod window;

/// Type alias for Result.
type TormResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> TormResult<()> {
    let mut torm = Torm::new()?;
    torm.start().await?;

    Ok(())
}
