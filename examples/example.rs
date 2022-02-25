use async_graph::*;

fn concat(inputs: (String, String)) -> String {
    format!("{} {}", inputs.0, inputs.1)
}

fn main() {
    let hello = "hello".to_owned();
    let mut name = Placeholder::<String>::new();

    let inputs = (hello, name.clone());
    let hello_name: Op<'_, (String, String), String> = Op::new(inputs, concat);

    let world = "world".to_string();
    name.provide(world);
    assert_eq!(hello_name.eval(), "hello world");

    let test = "test".to_string();
    name.provide(test);
    assert_eq!(hello_name.eval(), "hello test");
}
