use crate::utils;

pub fn execute() {
    let path_entries = utils::get_path_entries();

    println!("Current PATH entries:");
    for path in path_entries {
        println!("- {}", path.display());
    }
}
