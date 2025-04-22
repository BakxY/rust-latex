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

    let mut selected_template_str = "".to_string();

    loop {
        menu::display_template_selection(all_templates.clone());
        let selected_id = if let Ok(selected_id) = menu::select_template() {
            selected_id
        }
        else {
            continue;
        };

        let selected_template = if let Some(selected_template) = all_templates.get(selected_id - 1) {
            selected_template
        } else {
            continue;
        };

        selected_template_str = selected_template.to_string();

        break;
    }

    let (template_fields, template_groups) = config::get_all_template_fields(selected_template_str.clone());
    let selected_template_readable = config::get_human_readable_name(selected_template_str.clone());
    
    menu::display_field_menu(selected_template_readable);
    let filled_fields = menu::get_all_field_values(template_groups, template_fields);

    let path_to_template_tex_file = config::get_template_tex_location(selected_template_str.clone());

    let raw_latex_file = files::read_tex_template_file(path_to_template_tex_file);

    let filled_latex_file = files::populate_tex_template_fields(raw_latex_file, filled_fields);

    files::copy_all_template_files(selected_template_str);
    files::write_template(filled_latex_file);
}
