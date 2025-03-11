use ch036_unit_test::get_full_name;

#[test]
fn test_get_full_name_normal_input() {
    let result = get_full_name("John", "Smith");
    assert_eq!(result, "John Smith");
}

#[test]
#[should_panic]
fn test_get_full_name_firstname_special_chars() {
    get_full_name("*Jo&h^n", "Smith");
}

#[test]
#[should_panic]
fn test_get_full_name_lastname_special_chars() {
    get_full_name("John", "S^m*i&t|h");
}
