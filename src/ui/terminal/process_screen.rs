use crate::config::Config;
use crate::core::types::AppContext;
use crate::handle_cli_error;
use crate::logic::ocr::process_screenshot;
use std::io::Write;
use std::path::PathBuf;


pub fn input_screenshot(_: &mut AppContext) {
    let path = &Config::global().screenshot_path;

    print!("Введите путь: (Enter => {}): ", path.to_string_lossy());
    handle_cli_error!(std::io::stdout().flush());

    let mut input = String::default();
    handle_cli_error!(std::io::stdin().read_line(&mut input));

    let trimmed = input.trim();

    let target_path = if trimmed.is_empty() {
        path.clone()
    } else {
        PathBuf::from(input)
    };

    let confidence =  handle_cli_error!(process_screenshot(&target_path));

    println!("Точность: {}%", confidence)
}