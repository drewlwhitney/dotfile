use super::*;

#[cfg(test)]
mod create_package_system {
    // use super::super::super::package_system::PACKAGE_MANAGER_FILENAME;
    use super::*;

    #[test]
    fn works() {
        let test_folder = Path::new("./test_files/pac/create_package_system");
        let correct_folder = test_folder.join("correct"); // reference folder
        let temp_folder = test_folder.join("temp"); // where package system folder is stored
        let name = "pacman";
        let package_manager_folder = temp_folder.join(&name); // where the package system is stored

        // create the package system
        self::create_package_system(&temp_folder, name).unwrap();
        // check that all files were created
        fn read_directory(folder: impl AsRef<Path>) -> Vec<String> {
            fs::read_dir(folder)
                .unwrap()
                .map(|p| {
                    p.unwrap()
                        .path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                })
                .collect()
        }
        let correct_files = read_directory(&correct_folder);
        let package_manager_files = read_directory(&package_manager_folder);
        for file in &correct_files {
            assert!(package_manager_files.contains(file), "Missing file: {}", file);
        }
        // check the contents of the package manager file
        assert_eq!(
            fs::read_to_string(&correct_folder.join(package_system::PACKAGE_MANAGER_FILENAME))
                .unwrap(),
            fs::read_to_string(
                &package_manager_folder.join(package_system::PACKAGE_MANAGER_FILENAME)
            )
            .unwrap()
        );
        // cleanup
        fs::remove_dir_all(&temp_folder).unwrap();
    }
}
