use std::sync::{Arc, RwLock};

pub trait Node {
    type State;

    fn eval(&self) -> Self::State;
}

pub trait State: Sized {
    type Target;
}

#[macro_export]
macro_rules! state {
    ($t:ty) => {
        impl State for $t {
            type Target = $t;
        }

        impl Node for $t {
            type State = $t;

            fn eval(&self) -> $t {
                self.clone()
            }
        }
    };
}

state!(());
state!(String);

impl<T1, T2> State for (T1, T2)
where
    T1: State,
    T2: State,
{
    type Target = (T1::Target, T2::Target);
}

impl<T1, T2> Node for (T1, T2)
where
    T1: Node,
    T2: Node,
{
    type State = (T1::State, T2::State);

    fn eval(&self) -> Self::State {
        (self.0.eval(), self.1.eval())
    }
}

pub struct Op<'a, I, O> {
    input: Box<dyn Node<State = I> + 'a>,
    def: Box<dyn Fn(I) -> O + 'a>,
}

impl<'a, I, O> Op<'a, I, O> {
    pub fn new<N, F>(input: N, def: F) -> Self
    where
        N: Node<State = I> + 'a,
        F: Fn(I) -> O + 'a,
    {
        Self {
            input: Box::new(input),
            def: Box::new(def),
        }
    }
}

impl<'a, I, O> Node for Op<'a, I, O> {
    type State = O;

    fn eval(&self) -> O {
        let input = self.input.eval();
        (self.def)(input)
    }
}

#[derive(Clone)]
pub struct Placeholder<T> {
    state: Arc<RwLock<Option<T>>>,
}

impl<'a, T: Clone> Placeholder<T> {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(None)),
        }
    }

    pub fn provide(&mut self, value: T) {
        let mut state = self.state.write().expect("placeholder");
        *state = Some(value);
    }
}

impl<'a, T> State for Placeholder<T> {
    type Target = T;
}

impl<'a, T> Node for Placeholder<T>
where
    T: Clone,
{
    type State = T;

    fn eval(&self) -> Self::State {
        let state = self.state.read().expect("placeholder lock");
        state.as_ref().expect("input").clone()
    }
}
