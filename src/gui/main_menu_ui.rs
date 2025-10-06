use crate::game::GameSettings;
use crate::game::scene::Scene;
use glfw::Window;
use imgui::Ui;
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::imgui;

pub fn draw_main_menu(
    window: &mut Window,
    _scene: &mut Scene,
    _settings: &mut GameSettings,
    ui: &Ui,
    _is_key_esc: &mut bool,
    is_ingame: &mut bool,
) {
    let (_, height) = window.get_size();
    let height = height as f32;

    ui.window("Main menu")
        .title_bar(false)
        .position([0., height * 0.5 - 100.], imgui::Condition::Always)
        .size([0.0, 0.0], imgui::Condition::Once)
        .always_use_window_padding(true)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .build(|| {
            if ui.button_with_size("Start game", [200., 40.]) {
                *is_ingame = true;
            }
            ui.spacing();
            if ui.button_with_size("Exit", [200., 40.]) {
                window.set_should_close(true);
            }
        });
}
