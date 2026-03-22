use test_types::{schema_version, Event};

fn main() {
    let e = Event { kind: "A" };
    println!("flow-a: {} {}", e.kind, schema_version());
}