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
    let all_templates = files::get_templates();

    loop {
        menu::display_template_selection(all_templates.clone());
        let selected_id = menu::select_template();

        let selected_id = selected_id.unwrap_or_default();

        if selected_id.clone() == 0 {
            continue;
        }

        let selected_template = if let Some(selected_template) = all_templates.get(selected_id - 1) {
            selected_template
        } else {
            continue;
        };

        let selected_template = selected_template.to_string();

        config::get_human_readable_name(selected_template.clone());
        config::get_all_template_fields(selected_template);
    }
}
