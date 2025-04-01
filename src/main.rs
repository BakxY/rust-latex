/***
 * @file main.rs
 * @brief The main source file of the rust-latex project
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */

mod config;
mod files;
mod menu;

fn main() {
    loop {
        menu::display_template_selection();
        let selected_id = menu::select_template();

        //print!("{}", selected_id.unwrap());

        match selected_id {
            Ok(selected_id) => {
                print!("Leaving {}", selected_id);
                break;
            }
            _err => {
                continue;
            }
        }
    }

    config::get_human_readable_name("letter".to_string());
    config::get_all_template_fields("letter".to_string());
}
