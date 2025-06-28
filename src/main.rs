pub mod pac;

fn main() {
    pac::create_package_system("./psystems", "pacman").unwrap();
    pac::create_package_system("./psystems", "yay").unwrap();
}
