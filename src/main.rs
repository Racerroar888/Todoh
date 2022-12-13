use std::collections::HashMap;

fn add(mut todo: HashMap<u8, String>, item: String) -> HashMap<u8, String> {
    todo.insert(todo.len().try_into().unwrap(), item);
    return todo;
}

fn remove(mut todo: HashMap<u8, String>, key: &u8) -> HashMap<u8, String> {
	todo.remove(key);
	return todo;
}

fn main() {
    let mut todo: HashMap<u8, String> = HashMap::new();
    let test_string: String = String::from("Hello World!");

    todo = add(todo, test_string);

    println!("{:?}", todo);

		todo = remove(todo, &0);

		println!("{:?}", todo);
}
