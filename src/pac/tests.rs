use super::*;

const TEST_PACKAGE: &str = "trash-cli";
const TEST_PACKAGE2: &str = "nano";

fn set_up_file(name: &str, lines: &[&str]) -> PathBuf {
    let path = PathBuf::from(name);
    let mut file = File::create(&path).unwrap();
    for line in lines {
        writeln!(&mut file, "{}", line).unwrap();
    }
    return path;
}

fn clean_up_files(paths: &[PathBuf]) {
    for path in paths {
        fs::remove_file(path).expect("Failed to delete file");
    }
}

fn ensure_package_presence(packages: &[&str]) {
    let installed_packages =
        String::from_utf8(Command::new("pacman").arg("-Qqen").output().unwrap().stdout).unwrap();
    let installed_packages: Vec<&str> = installed_packages.lines().collect();

    for package in packages {
        if !installed_packages.contains(package) {
            panic!("Unable to test, {} is not installed", package);
        }
    }
}

#[cfg(test)]
mod download {
    use super::*;

    #[test]
    /// This does not actually test that the package was installed because that requires user input.
    fn works() {
        // set up file
        let package_file = set_up_file("test.txt", &[TEST_PACKAGE]);

        // set up package system
        let mut install_command = Command::new("pacman");
        install_command.arg("-S");
        let mut package_system = PackageSystem::new(
            package_file.to_str().unwrap(),
            "",
            Command::new(""),
            install_command,
        );

        // run the command
        download(&mut package_system).unwrap();

        // cleanup
        clean_up_files(&[package_file]);
    }
}

#[cfg(test)]
mod upload {
    use super::*;

    #[test]
    fn works() {
        // make sure the test package is installed
        ensure_package_presence(&[TEST_PACKAGE]);

        // set up files
        let excluded_packages_file = set_up_file("exclude.txt", &[TEST_PACKAGE]);
        let package_file = "test.txt";

        // set up package system
        let mut list_command = Command::new("pacman");
        list_command.arg("-Qqen");
        let mut package_system = PackageSystem::new(
            package_file,
            excluded_packages_file.to_str().unwrap(),
            list_command,
            Command::new(""),
        );

        // run the command
        upload(&mut package_system).unwrap();

        // check that the exclusion functionality works
        let contents = fs::read_to_string(package_file).unwrap();
        let contents: Vec<&str> = contents.lines().collect();
        if contents.contains(&TEST_PACKAGE) {
            panic!("An excluded package was not properly excluded");
        }

        // cleanup
        clean_up_files(&[excluded_packages_file, PathBuf::from(package_file)]);
    }
}

#[cfg(test)]
mod exclude {
    use super::*;

    #[test]
    fn works() {
        let filename = "excluded.txt";
        // set up file
        let excluded_packages_file = set_up_file(filename, &[TEST_PACKAGE]);

        // run the command
        exclude(&excluded_packages_file, &vec![TEST_PACKAGE2]);

        // make sure it works
        let contents =
            fs::read_to_string(&excluded_packages_file).expect("Failed to read from file");
        let contents: Vec<&str> = contents.lines().collect();
        if !contents.contains(&TEST_PACKAGE2) {
            panic!("The package was not excluded");
        }

        // -----------------------------------------------------------------------------------------

        let excluded_packages_file = set_up_file(filename, &[TEST_PACKAGE, TEST_PACKAGE2]);
        let excluded_packages = vec![TEST_PACKAGE];
        exclude(&excluded_packages_file, &excluded_packages);

        let contents = fs::read_to_string(&excluded_packages_file).unwrap();
        let contents: Vec<&str> = contents.lines().collect();

        for i in 0..contents.len() {
            if excluded_packages[i] != contents[i] {
                panic!("File contents were changed");
            }
        }

        // cleanup
        clean_up_files(&[excluded_packages_file]);
    }

    #[test]
    fn multiple_packages() {
        let excluded_packages_file = set_up_file("excluded.txt", &[]);
        let excluded_packages = vec![TEST_PACKAGE, TEST_PACKAGE2];

        exclude(&excluded_packages_file, &excluded_packages);

        let contents = fs::read_to_string(&excluded_packages_file).unwrap();
        let contents: Vec<&str> = contents.lines().collect();
        for package in excluded_packages {
            if !contents.contains(&package) {
                panic!("A package was not excluded")
            }
        }

        clean_up_files(&[excluded_packages_file]);
    }
}

#[cfg(test)]
mod reinclude {
    use super::*;

    #[test]
    fn works() {}
}
