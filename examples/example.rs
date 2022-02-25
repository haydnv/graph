use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::fmt::{Display, Formatter};

use async_graph::*;
use hr_id::Id;

#[derive(Debug)]
struct Error(String);

impl std::error::Error for Error {}

impl async_graph::Error for Error {
    fn consume(self, id: &Id) -> Self {
        todo!()
    }

    fn collision<E: Display, N: Display>(id: &Id, existing: E, new: N) -> Self {
        Self(format!(
            "there is already a value {} for {} (not {})",
            existing, id, new
        ))
    }

    fn not_found(id: &Id) -> Self {
        Self(format!("not found: {}", id))
    }

    fn invalid_type<E: fmt::Display, A: fmt::Display>(expected: E, actual: A) -> Self {
        Self(format!("invalid type {}, expected {}", actual, expected))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Clone)]
enum State {
    Number(i32),
    String(String),
}

impl async_graph::State for State {
    type Target = Self;
}

impl From<String> for State {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl TryFrom<State> for i32 {
    type Error = Error;

    fn try_from(value: State) -> Result<Self, Self::Error> {
        match value {
            State::Number(i) => Ok(i),
            other => Err(async_graph::Error::invalid_type("i32", other)),
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(i) => fmt::Display::fmt(i, f),
            Self::String(s) => fmt::Display::fmt(s, f),
        }
    }
}

fn concat(inputs: (String, String)) -> String {
    format!("{} {}", inputs.0, inputs.1)
}

fn main() {
    let mut graph = Graph::<State, Error>::new();

    graph
        .add_state("hello".parse().expect("id"), "hello".to_string().into())
        .unwrap();
}
