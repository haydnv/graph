use std::convert::Infallible;

use async_trait::async_trait;

use async_graph::Op;

struct Add {
    l: i32,
    r: i32,
}

#[async_trait]
impl Op<()> for Add {
    type Input = (i32, i32);
    type Output = i32;
    type Error = Infallible;

    fn new(input: Self::Input) -> Self {
        let (l, r) = input;
        Self { l, r }
    }

    async fn execute(&self, _cxt: ()) -> Result<Self::Output, Self::Error> {
        Ok(self.l + self.r)
    }
}

#[tokio::main]
async fn main() {
    let op = Add::new((2, 2));
    assert_eq!(op.execute(()).await, Ok(4));
}
