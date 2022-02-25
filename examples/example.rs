use async_graph::*;

fn concat(inputs: (String, String)) -> String {
    format!("{} {}", inputs.0, inputs.1)
}

fn main() {
    let hello = "hello".to_owned();
    let world = "world".to_owned();
    let hello_world = Op::new((hello, world), concat);
    assert_eq!(hello_world.eval(), "hello world");
}
