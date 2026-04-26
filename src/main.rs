use parser::LsParser;

mod parser;

// use std::path::Path;
// use std::{env, process::exit};

// use rust_ls::{
//     Config, print_btree, print_newline, print_space, read_contents_of_given_directory,
//     read_list_recursive, read_recursive,
// };
// /*
//  * parser without
//  *
//  * - conflicting args like -a or -A
//  * - validation phase
//  * - error handling stratergy
//  * - add one value option (state machine)
//  */
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let config = parse_ls_arguments(&args);

//     for str_path in &config.files {
//         let path = Path::new(str_path);

//         if config.print_list_format {
//             let contents = read_list_recursive(&path, &config).unwrap();
//             print_btree(&contents, &config);
//         } else if config.show_subdirectories_content {
//             let contents = read_recursive(&path, &config).unwrap();
//             print_btree(&contents, &config);
//         } else {
//             let mut contents = read_contents_of_given_directory(&path).unwrap();
//             if config.display_directory_order == false {
//                 contents.sort_by(|a, b| {
//                     let a_str = a.to_string_lossy().to_lowercase();
//                     let b_str = b.to_string_lossy().to_lowercase();

//                     // Strip the leading dot if it exists for the sake of comparison
//                     let a_normalized = a_str.strip_prefix('.').unwrap_or(&a_str);
//                     let b_normalized = b_str.strip_prefix('.').unwrap_or(&b_str);

//                     a_normalized.cmp(b_normalized)
//                 });
//             }
//             if config.print_reverse {
//                 if config.display_directory_order == false {
//                     contents.sort_by(|a, b| {
//                         let a_str = a.to_string_lossy().to_lowercase();
//                         let b_str = b.to_string_lossy().to_lowercase();

//                         // Strip the leading dot if it exists for the sake of comparison
//                         let a_normalized = a_str.strip_prefix('.').unwrap_or(&a_str);
//                         let b_normalized = b_str.strip_prefix('.').unwrap_or(&b_str);

//                         a_normalized.cmp(b_normalized)
//                     });

//                     contents.reverse();
//                 }
//             }

//             println!("{str_path}:");
//             if config.newline {
//                 print_newline(
//                     &contents,
//                     config.almost_all || config.display_directory_order,
//                 );
//             } else {
//                 print_space(
//                     &contents,
//                     config.almost_all || config.display_directory_order,
//                 );
//             }
//             println!("\n");
//         }
//     }
// }

// fn parse_ls_arguments(args: &Vec<String>) -> Config {
//     let mut config = Config::new();
//     for arg in args.into_iter().skip(1) {
//         if arg.starts_with("---") {
//             exit(1);
//         }

//         if arg.starts_with("--") {
//             match arg.as_str() {
//                 "--almost-all" => config.almost_all = true,
//                 "--human-readable" => config.human_readable_size = true,
//                 "--reverse" => config.print_reverse = true,
//                 "--recursive" => config.show_subdirectories_content = true,
//                 _ => {
//                     eprintln!("Unknown flag {arg}");
//                     exit(1);
//                 }
//             }
//             continue;
//         } else if arg.starts_with("-") {
//             for ch in arg.chars().skip(1) {
//                 match ch {
//                     'A' => config.almost_all = true,
//                     '1' => config.newline = true,
//                     'l' => config.print_list_format = true,
//                     'f' => config.display_directory_order = true,
//                     'h' => config.human_readable_size = true,
//                     'r' => config.print_reverse = true,
//                     'R' => config.show_subdirectories_content = true,
//                     _ => {
//                         eprintln!("Unknown flag {ch}");
//                         exit(1);
//                     }
//                 }
//             }
//             continue;
//         } else {
//             config.files.push(arg.clone());
//         }
//     }

//     if config.files.is_empty() {
//         config.files.push(String::from("."));
//     }

//     config
// }

fn main() {
    let ls_parser = LsParser::new("ls");

    let config = ls_parser.parse_args();
    println!("{:?}", config);
}
