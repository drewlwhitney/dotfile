use std::path::Path;

use super::read_file_to_vector;

/// Tests `read_file_to_vector()`
#[test]
fn it_works() {
    let test_file = Path::new(file!()).parent().unwrap().join("test_file.txt");
    assert_eq!(read_file_to_vector(&test_file).unwrap(), vec!["ONE", "TWO", "THREE", "FOUR"]);
}
