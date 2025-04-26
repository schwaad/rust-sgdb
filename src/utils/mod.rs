use std::path::PathBuf;

pub fn get_data_sets_dir_path() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut path = PathBuf::from(manifest_dir);
    path.push("data_sets");
    path
}

pub fn get_data_set_file_path(data_set_name: &str) -> PathBuf {
    let mut path = get_data_sets_dir_path();
    path.push(format!("{}.csv", data_set_name));
    path
}

