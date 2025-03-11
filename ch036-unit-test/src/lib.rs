/// ```
/// use ch036_unit_test::get_full_name;
/// let full_name = get_full_name("John", "Smith");
/// assert_eq!(full_name, "John Smith");
/// ```
pub fn get_full_name(firstname: &str, lastname: &str) -> String {
    let pattern = &['^', '$', '*', '&', '|', '+', '-'];
    if firstname.contains(pattern) {
        panic!("First name cannot contain special characters");
    }
    if lastname.contains(pattern) {
        panic!("Last name cannot contain special characters");
    }

    let mut result = "".to_string();
    result.push_str(firstname);
    result.push_str(" ");
    result.push_str(lastname);
    result
}
