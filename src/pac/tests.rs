use super::*;
use std::fs;

#[cfg(test)]
mod package_system {
    use super::*;

    #[cfg(test)]
    mod exclude {
        use super::*;

        #[test]
        fn works() {}

        #[test]
        fn multiple_packages() {}
    }

    #[cfg(test)]
    mod reinclude {
        use super::*;

        #[test]
        fn works() {}

        #[test]
        fn multiple_packages() {}
    }
}

// #[cfg(test)]
// mod exclude {
//     use super::*;

//     #[test]
//     fn works() {
//         let filename = "excluded.txt";
//         // set up file
//         let excluded_packages_file = set_up_file(filename, &[TEST_PACKAGE]);

//         // run the command
//         exclude(&excluded_packages_file, &vec![TEST_PACKAGE2]);

//         // make sure it works
//         let contents =
//             fs::read_to_string(&excluded_packages_file).expect("Failed to read from file");
//         let contents: Vec<&str> = contents.lines().collect();
//         if !contents.contains(&TEST_PACKAGE2) {
//             panic!("The package was not excluded");
//         }

//         //
// -----------------------------------------------------------------------------------------

//         let excluded_packages_file = set_up_file(filename, &[TEST_PACKAGE, TEST_PACKAGE2]);
//         let excluded_packages = vec![TEST_PACKAGE];
//         exclude(&excluded_packages_file, &excluded_packages);

//         let contents = fs::read_to_string(&excluded_packages_file).unwrap();
//         let contents: Vec<&str> = contents.lines().collect();

//         for i in 0..contents.len() {
//             if excluded_packages[i] != contents[i] {
//                 panic!("File contents were changed");
//             }
//         }

//         // cleanup
//         clean_up_files(&[excluded_packages_file]);
//     }

//     #[test]
//     fn multiple_packages() {
//         let excluded_packages_file = set_up_file("excluded.txt", &[]);
//         let excluded_packages = vec![TEST_PACKAGE, TEST_PACKAGE2];

//         exclude(&excluded_packages_file, &excluded_packages);

//         let contents = fs::read_to_string(&excluded_packages_file).unwrap();
//         let contents: Vec<&str> = contents.lines().collect();
//         for package in excluded_packages {
//             if !contents.contains(&package) {
//                 panic!("A package was not excluded")
//             }
//         }

//         clean_up_files(&[excluded_packages_file]);
//     }
// }
