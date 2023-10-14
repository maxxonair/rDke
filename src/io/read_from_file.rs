
use std::fs;


fn read_from_file(file_path_in: String) -> String {
  fs::read_to_string(file_path_in)
    .expect("Unable to read file")
}