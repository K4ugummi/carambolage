use crate::game::scene::Scene;
use crate::game::GameSettings;
use glfw::Window;
use imgui::{im_str, ImGuiCol, Ui};
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::imgui;
use nalgebra::clamp;

pub(super) fn draw_game_ui(
    window: &mut Window,
    scene: &mut Scene,
    settings: &mut GameSettings,
    ui: &Ui,
    is_ingame_menu: &mut bool,
    is_key_esc: &mut bool,
    is_ingame: &mut bool,
) {
    let (width, height) = window.get_size();
    let width = width as f32;
    let height = height as f32;

    let mut should_close = false;

    for (id, car) in scene.cars.iter().enumerate() {
        ui.window(im_str!("Player {}", id + 1))
            .title_bar(true)
            .position(player_ui_pos(width, height, id), imgui::ImGuiCond::Always)
            .size((250.0, 0.0), imgui::ImGuiCond::Once)
            .always_use_window_padding(true)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .build(|| {
                ui.with_color_var(ImGuiCol::PlotHistogram, boost_to_rgba(car.boost), || {
                    ui.progress_bar(car.boost / 100.)
                        .overlay_text(im_str!("BOOST"))
                        .size((-1., 40.))
                        .build();
                });
            });
    }

    let mut close_ingame_menu = false;
    if !*is_key_esc && window.get_key(glfw::Key::Escape) == glfw::Action::Press {
        if *is_ingame_menu {
            close_ingame_menu = true;
        }
        *is_ingame_menu = !*is_ingame_menu;
        *is_key_esc = true;
    } else if *is_key_esc && window.get_key(glfw::Key::Escape) == glfw::Action::Release {
        *is_key_esc = false;
    }

    let mut is_smooth_zoom = scene.camera.is_smooth_zoom;
    let mut is_smooth_pan = scene.camera.is_smooth_pan;
    if *is_ingame_menu {
        ui.open_popup(im_str!("Menu"));
    }

    ui.popup_modal(im_str!("Menu"))
        .title_bar(false)
        .always_use_window_padding(true)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .build(|| {
            if ui.button(im_str!("Continue"), (200., 40.)) || close_ingame_menu {
                ui.close_current_popup();
                *is_ingame_menu = false;
            }
            ui.separator();
            ui.text(im_str!("Camera settings:"));
            ui.checkbox(im_str!("Smooth zoom"), &mut is_smooth_zoom);
            ui.checkbox(im_str!("Smooth pan"), &mut is_smooth_pan);
            ui.separator();
            ui.input_float(im_str!("Gamma"), &mut settings.gamma).step(0.1).build();
            ui.separator();
            if ui.button(im_str!("Main menu"), (200., 40.)) {
                *is_ingame_menu = false;
                *is_ingame = false;
                scene.reset_cars();
            }
            ui.separator();
            if ui.button(im_str!("Exit"), (200., 40.)) {
                should_close = true;
            }
        });
    settings.gamma = clamp(settings.gamma, 0.5, 2.5);

    scene.camera.is_smooth_zoom = is_smooth_zoom;
    scene.camera.is_smooth_pan = is_smooth_pan;

    window.set_should_close(should_close);
}

fn boost_to_rgba(boost: f32) -> (f32, f32, f32, f32) {
    let bst = boost * 0.01;
    (1.0 - bst, clamp(bst, 0.0, 0.77), 0.0, 1.0)
}
fn player_ui_pos(width: f32, height: f32, id: usize) -> (f32, f32) {
    match id {
        0 => (20.0, height - 100.),
        1 => (width - 270., height - 100.),
        _ => unreachable!(),
    }
}
