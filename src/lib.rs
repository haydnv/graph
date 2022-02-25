pub trait Node {
    type State;

    fn eval(&self) -> Self::State;
}

pub trait State {}

impl State for () {}
impl State for String {}

impl<T1, T2> State for (T1, T2)
    where
        T1: State,
        T2: State,
{
}

impl<T> Node for T
    where
        T: State + Clone,
{
    type State = Self;

    fn eval(&self) -> Self {
        self.clone()
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
