use async_graph::*;

fn concat(inputs: (String, String)) -> String {
    format!("{} {}", inputs.0, inputs.1)
}

fn main() {
    let hello = "hello".to_owned();
    let name = Placeholder::<String>::new();

    let inputs = (hello, name);
    let hello_name: Op<'_, (String, String), String> = Op::new(inputs, concat);

    let mut graph = Graph::new(name, hello_name);
    assert_eq!(graph.eval("world".to_string()), "hello world");
}
