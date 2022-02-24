use std::convert::{TryFrom, TryInto};
use std::pin::Pin;
use std::process::Output;

use async_trait::async_trait;
use futures::{Future, FutureExt, TryFutureExt};

trait OpDef<Context, I, O, E>: Fn(Context, I) {}

pub enum ErrorKind {
    TypeError,
}

pub struct Error {
    kind: ErrorKind,
    message: String,
}

pub struct Op<Context, I, O, E> {
    input: Node<Context, I>,
    def: Box<dyn Fn(Context, I) -> Result<O, E> + Send>,
}

impl<Context, I, O, E> Op<Context, I, O, E>
where
    Context: Clone,
    I: Clone,
    Error: From<E>,
{
    fn eval(&self, context: Context) -> Pin<Box<dyn Future<Output = Result<O, Error>> + '_>> {
        Box::pin(async move {
            let input = self.input.eval(context.clone()).await?;
            (self.def)(context, input).map_err(Error::from)
        })
    }
}

pub enum Node<Context, State> {
    State(State),
    Op(Box<Op<Context, State, State, Error>>),
}

impl<Context, State> Node<Context, State>
where
    Context: Clone,
    State: Clone,
{
    fn eval(&self, context: Context) -> Pin<Box<dyn Future<Output = Result<State, Error>> + '_>> {
        Box::pin(async move {
            match self {
                Self::State(state) => Ok(state.clone()),
                Self::Op(op) => op.eval(context).await,
            }
        })
    }
}
