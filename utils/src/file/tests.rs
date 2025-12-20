use std::path::Path;

use super::read_file_to_hashset;

/// Tests `read_file_to_vector()`
#[test]
fn it_works() {
    let test_file = Path::new(file!()).parent().unwrap().join("test_file.txt");
    assert_eq!(read_file_to_hashset(&test_file).unwrap(), ["ONE", "TWO", "THREE", "FOUR"].map(String::from).into());
}
