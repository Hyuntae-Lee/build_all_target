use file_hdl::read_file;

pub fn get_version(ver_file_path : &str, label_ver_a : &str, label_ver_b : &str,
                   label_ver_c : &str, label_ver_d : &str) -> String {
    // read disp file
    let mut contents = String::new();
    if !read_file(&mut contents, ver_file_path) {
        println!("[PRJ_INFO] Version file read fail!");
    }

    //
    let mut iter_ws = contents.split_whitespace();

    iter_ws.find(|&word| word == label_ver_a);
    let ver_a = match iter_ws.next() {
        Some(x) => x,
        None => ""
    };

    iter_ws.find(|&word| word == label_ver_b);
    let ver_b = match iter_ws.next() {
        Some(x) => x,
        None => ""
    };

    iter_ws.find(|&word| word == label_ver_c);
    let ver_c = match iter_ws.next() {
        Some(x) => x,
        None => ""
    };

    iter_ws.find(|&word| word == label_ver_d);
    let mut ver_d = match iter_ws.next() {
        Some(x) => x,
        None => ""
    }.to_string();
    ver_d.remove(0);
    ver_d.remove(1);

    return format!("V{}.{}.{}{}", ver_a, ver_b, ver_c, ver_d);
}

pub fn get_target_list(target_list : &mut Vec<String>, disp_file_path : &str,
                       label_prefix : &str) -> bool {
    // read disp file
    let mut contents = String::new();
    if !read_file(&mut contents, disp_file_path) {
        println!("[PRJ_INFO] List file read fail!");
    }

    // get model list
    // - make string to a list of words
    let iter_ws = contents.split_whitespace();
    // - filter
    let iter_len = iter_ws.filter(|w| w.chars().count() > label_prefix.len());
    let iter_pfx = iter_len.filter(|w| w.to_string().split_at(label_prefix.len()).0 == label_prefix);
    let iter_str = iter_pfx.map(|w| w.to_string());
    // - collect model names
    target_list.extend(iter_str);

    return true;
}

pub fn get_model_setting_txt(main_file_path : &str, setting_fn_name : &str) -> Option<String> {
    // read main file
    let mut contents = String::new();
    if !read_file(&mut contents, main_file_path) {
        println!("[PRJ_INFO] Main file read fail!");
    }

    let text_idx = match contents.find(setting_fn_name) {
        None => {
            return None;
        },
        Some(x) => x
    };

    let text_len = match contents.split_at(text_idx).1.find(";") {
        None => {
            return None;
        },
        Some(x) => x
    };

    let head_cleared = contents.split_off(text_idx);
    let tail_cleared = head_cleared.split_at(text_len).0;

    return Some(tail_cleared.to_string());
}
