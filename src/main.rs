use std::collections::HashMap;

fn add(mut todo: HashMap<u8, String>, item: String) -> HashMap<u8, String> {
    todo.insert(todo.len().try_into().unwrap(), item);
    return todo;
}

fn remove(mut todo: HashMap<u8, String>, key: &u8) -> HashMap<u8, String> {
	todo.remove(key);
	return todo;
}

fn print(todo: &HashMap<u8, String>) {
	for (key, item) in todo {
		println!("{key}: {item}")
	}
}

fn main() {
    let mut todo: HashMap<u8, String> = HashMap::new();
    let test_string: String = String::from("Hello World!");
	let test_string_two: String = String::from("Testing");

    todo = add(todo, test_string);
    todo = add(todo, test_string_two);


		print(&todo);

		todo = remove(todo, &0);

		print(&todo);
}
