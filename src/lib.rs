use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt;

use hr_id::Id;

pub trait Error: std::error::Error {
    fn consume(self, id: &Id) -> Self;

    fn collision<E: fmt::Display, N: fmt::Display>(id: &Id, existing: E, new: N) -> Self;

    fn not_found(id: &Id) -> Self;

    fn invalid_type<E: fmt::Display, A: fmt::Display>(expected: E, actual: A) -> Self;
}

pub trait Node {
    type State;
    type Error;

    fn eval(&self) -> Result<Self::State, Self::Error>;
}

pub trait State: Sized {
    type Target;
}

impl<T> Node for T
where
    T: State<Target = T> + Clone,
{
    type State = T;
    type Error = Infallible;

    fn eval(&self) -> Result<T, Infallible> {
        Ok(self.clone())
    }
}

impl State for () {
    type Target = ();
}

impl State for String {
    type Target = String;
}

pub struct Op<'a, I, O, E> {
    input: Box<dyn Node<State = I, Error = E> + 'a>,
    def: Box<dyn Fn(I) -> Result<O, E> + 'a>,
}

impl<'a, I, O, E> Op<'a, I, O, E> {
    pub fn new<N, F>(input: N, def: F) -> Self
    where
        N: Node<State = I, Error = E> + 'a,
        F: Fn(I) -> Result<O, E> + 'a,
    {
        Self {
            input: Box::new(input),
            def: Box::new(def),
        }
    }
}

impl<'a, I, O, E> Node for Op<'a, I, O, E> {
    type State = O;
    type Error = E;

    fn eval(&self) -> Result<O, E> {
        let input = self.input.eval()?;
        (self.def)(input)
    }
}

trait Placeholder {}

impl Placeholder for Id {}

impl Placeholder for (Id, Id) {}

trait Resolve<'a, T, E> {
    fn resolve<P: Placeholder>(
        &self,
        placeholder: P,
    ) -> Option<Box<dyn Node<State = T, Error = E>>>;
}

pub struct Graph<'a, T, E> {
    states: HashMap<Id, T>,
    ops: HashMap<Id, (Box<dyn Placeholder>, Box<dyn Fn(T) -> Result<T, E> + 'a>)>,
}

impl<'a, T, E> Graph<'a, T, E>
where
    T: State + Clone + fmt::Display,
    E: Error,
{
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            ops: HashMap::new(),
        }
    }

    pub fn add_state(&mut self, id: Id, state: T) -> Result<(), E> {
        match self.states.entry(id) {
            Entry::Vacant(vacant) => {
                vacant.insert(state);
                Ok(())
            }
            Entry::Occupied(existing) => Err(E::collision(existing.key(), existing.get(), state)),
        }
    }

    pub fn build(self, inputs: HashMap<Id, T>) -> Box<dyn Node<State = T, Error = E>> {
        todo!()
    }
}
