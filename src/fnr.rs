use file_hdl::{read_file, write_to_file};

pub fn find_text_and_replace(file_path : &str,
                             text_to_find : &str,
                             text_to_replace_with : &str) -> bool {
    // read
    let mut contents = String::new();

    if !read_file(&mut contents, file_path) {
        println!("[FNR] File read fail!");
        return false;
    }

    // replace string
    let new_contents = contents.replace(text_to_find, text_to_replace_with);

    // write to file
    if !write_to_file(file_path, new_contents.as_str()) {
        println!("[FNR] File write fail!");
        return false;
    }

    return true;
}
