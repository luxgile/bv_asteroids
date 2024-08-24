mod hittable_button;
mod main_menu;

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(main_menu::plugin);
}
