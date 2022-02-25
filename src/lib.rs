use std::marker::PhantomData;

pub trait Node {
    type State;

    fn eval(&self) -> Self::State;
}

pub trait State: Sized {
    type Target;
}

macro_rules! rust_state {
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

rust_state!(());
rust_state!(String);

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

pub struct Placeholder<T> {
    state: PhantomData<T>,
}

impl<T> Copy for Placeholder<T> {}

impl<T> Clone for Placeholder<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Placeholder<T> {
    pub fn new() -> Self {
        Self { state: PhantomData }
    }
}

impl<T> State for Placeholder<T> {
    type Target = T;
}

impl<T> Node for Placeholder<T> {
    type State = T;

    fn eval(&self) -> Self::State {
        panic!("cannot evaluate a placeholder")
    }
}

pub struct Graph<'a, Input, Output> {
    input: Box<dyn Node<State = Input> + 'a>,
    output: Box<dyn Node<State = Output> + 'a>,
}

impl<'a, Input, Output> Graph<'a, Input, Output>
where
    Input: 'a,
    Output: 'a,
{
    pub fn new<O>(input: Placeholder<Input>, output: O) -> Self
    where
        O: Node<State = Output> + 'a,
        Placeholder<Input>: Node<State = Input>,
    {
        let input: Box<dyn Node<State = Input> + 'a> = Box::new(input);

        Self {
            input,
            output: Box::new(output),
        }
    }

    pub fn eval<I>(&mut self, input: I) -> Output
    where
        I: Node<State = Input> + 'a,
    {
        self.input = Box::new(input);
        self.output.eval()
    }
}
