use crate::game::scene::Scene;
use crate::game::GameSettings;
use glfw::Window;
use imgui::{im_str, ImGuiCol, Ui};
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::imgui;
use nalgebra::clamp;

pub fn draw_main_menu(
    window: &mut Window,
    scene: &mut Scene,
    settings: &mut GameSettings,
    ui: &Ui,
    is_key_esc: &mut bool,
    is_ingame: &mut bool,
) {
    ui.window(im_str!("Main menu"))
        .title_bar(false)
        .position((0.,0.), imgui::ImGuiCond::Always)
        .size((0.0, 0.0), imgui::ImGuiCond::Once)
        .always_use_window_padding(true)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .build(|| {
            if ui.button(im_str!("Start game"), (200., 40.)) {
                *is_ingame = true;
            }
        });
}
