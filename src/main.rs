use torm::Torm;

mod event;
mod state;
mod torm;

/// Type alias for Result.
type TormResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> TormResult<()> {
    let mut torm = Torm::new()?;
    torm.start().await?;

    Ok(())
}
