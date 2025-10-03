use gtk::prelude::*;

fn main() {
    // Store `i32` as `Value`
    let integer_value = 10.to_value();

    // Retrieve `i32` from `Value`
    let integer = integer_value
        .get::<i32>()
        .expect("The value needs to be of type `i32`.");

    // Check if the retrieved value is correct
    assert_eq!(integer, 10);

    // Store string as `Value`
    let string_value = "Hello!".to_value();

    // Retrieve `String` from `Value`
    let string = string_value
        .get::<String>()
        .expect("The value needs to be of type `String`.");

    // Check if the retrieved value is correct
    assert_eq!(string, "Hello!".to_string());

    // Store `Option<String>` as `Value`
    let string_some_value = "Hello!".to_value();
    let string_none_value = None::<String>.to_value();

    // Retrieve `String` from `Value`
    let string_some = string_some_value
        .get::<Option<String>>()
        .expect("The value needs to be of type `Option<String>`.");
    let string_none = string_none_value
        .get::<Option<String>>()
        .expect("The value needs to be of type `Option<String>`.");

    // Check if the retrieved value is correct
    assert_eq!(string_some, Some("Hello!".to_string()));
    assert_eq!(string_none, None);
    //
    // Store `i32` as `Variant`
    let integer_variant = 10.to_variant();

    // Retrieve `i32` from `Variant`
    let integer = integer_variant
        .get::<i32>()
        .expect("The variant needs to be of type `i32`.");

    // Check if the retrieved value is correct
    assert_eq!(integer, 10);

    let variant = vec!["Hello", "there!"].to_variant();
    assert_eq!(variant.n_children(), 2);
    let vec = &variant
        .get::<Vec<String>>()
        .expect("The variant needs to be of type `String`.");
    assert_eq!(vec[0], "Hello");
}
