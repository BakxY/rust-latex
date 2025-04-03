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

    let (template_fields, template_groups) = config::get_all_template_fields(selected_template_str);
    let mut template_fields_mod = template_fields;

    for group in template_groups {
        let (template_fields_cleaned, current_group_fields) = config::get_group_fields(template_fields_mod, group);

        template_fields_mod = template_fields_cleaned;

        for field in current_group_fields {
            println!("{} {} {}", field.group, field.readable, field.replace);
        }
    }

    print!("{}", template_fields_mod.len());
}
