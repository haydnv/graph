use std::error::Error;

use async_trait::async_trait;

#[async_trait]
pub trait Op<Context> {
    type Input: Send + Sync;
    type Output: Send + Sync;
    type Error: Error + Sized + Send + Sync;

    fn new(input: Self::Input) -> Self;

    async fn execute(&self, context: Context) -> Result<Self::Output, Self::Error>;
}
