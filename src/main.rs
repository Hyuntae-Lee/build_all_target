mod file_hdl;
mod fnr;
mod prj_info;

use std::env;
use std::process::Command;
use std::fs;

fn main() {
    // parse arguments
    let args: Vec<String> = env::args().collect();
    let (ver_file_path, disp_file_path, main_file_path, out_file_path, out_dir_path,
        label_ver_a, label_ver_b, label_ver_c, label_ver_d, fn_model_set, model_label_prefix)
        = match parse_input(&args) {
        Err(r) => {
            println!("{:?}", r);
            return;
        },
        Ok(x) => x
    };

    // get Version
    let ver_str = prj_info::get_version(ver_file_path, label_ver_a, label_ver_b, label_ver_c,
        label_ver_d);
    println!("Version : {}", ver_str);

    // get target list
    let mut target_list : Vec<String> = Vec::new();
    if !prj_info::get_target_list(&mut target_list, disp_file_path, model_label_prefix) {
        return;
    }

    // get output
    prepare_to_build(out_dir_path);
    for target in target_list {
        let mut model_str = target.clone();
        let out_file_name = model_str.split_off(model_label_prefix.len());

        // display progress
        println!("... {}", out_file_name);

        // build
        if build_for_target(&target, fn_model_set, main_file_path) != true {
            continue;
        }

        // collect output
        let out_bin_file_path = format!("{}\\{}_{}.bin", out_dir_path, out_file_name, ver_str);
        if let Err(_) = fs::copy(out_file_path, out_bin_file_path) {
            continue;
        }
    }
}

fn parse_input<'a>(input : &'a Vec<String>)
    -> Result<(&'a str, &'a str, &'a str, &'a str, &'a str, &'a str, &'a str, &'a str, &'a str,
               &'a str, &'a str), &'a str> {
    if input.capacity() != 12 {
        return Err("Two few input!");
    }

    return Ok((&input[1], &input[2], &input[3], &input[4], &input[5], &input[6], &input[7],
            &input[8], &input[9], &input[10], &input[11]));
}

fn prepare_to_build(out_dir_path : &str) {
    if let Err(_) = fs::create_dir(out_dir_path) {
        // do nothing
    }
    // - rebuild project
    println!("Rebuild project ...");
    Command::new("clean_build_workspace.bat")
            .output()
            .expect("failed to execute process");
}

fn build_for_target(target : &str, fn_model_set : &str, main_file_path : &str) -> bool {
    // change target
    let target_set_txt = format!("{}({})", fn_model_set, target);
    let cur_target_set_txt
        = match prj_info::get_model_setting_txt(main_file_path, fn_model_set) {
        None => {
            return false;
        },
        Some(x) => x
    };
    fnr::find_text_and_replace(main_file_path, &cur_target_set_txt, &target_set_txt);

    // build
    Command::new("build_workspace.bat")
            .output()
            .expect("failed to execute process");

    return true;
}
