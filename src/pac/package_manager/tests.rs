use super::*;

/// Tests `from_toml_string()`.
#[cfg(test)]
mod toml_parsing {
    use super::*;
    /// Test a valid package manager file.
    #[test]
    fn valid_package_manager() {
        let package_manager = PackageManager::from_toml_string(include_str!("./files/valid.toml")).unwrap();
        // check install command
        assert_eq!(package_manager.install_command.get_program(), "sudo");
        assert_eq!(package_manager.install_command.get_args().collect::<Vec<_>>(), vec!["pacman", "-S", "--needed", "--noconfirm"]);
        // check list command
        assert_eq!(package_manager.list_command.get_program(), "pacman");
        assert_eq!(package_manager.list_command.get_args().collect::<Vec<_>>(), ["-Qqen"]);
    }

    /// Test an invalid package manager file.
    #[test]
    #[should_panic]
    fn invalid_package_manager() {
        PackageManager::from_toml_string(include_str!("./files/invalid.toml")).unwrap();
    }
}
