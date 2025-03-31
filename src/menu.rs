/**
 * @file menu.rs
 * @brief A lib file to handle all menus
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */

use super::files;

pub fn display_template_selection() {
    println!("-------------------- RUST-LATEX --------------------\n");
    println!("{}\n", files::get_template_location());
    println!("--------------------- TEMPLATES --------------------\n");
}
