mod game_ui;
mod main_menu_ui;

use self::{game_ui::*, main_menu_ui::*};
use crate::game::scene::Scene;
use crate::game::GameSettings;
use glfw::{Window, WindowEvent};
use imgui::{FontGlyphRange, ImFontConfig, ImGui, ImGuiCol, ImVec2, ImVec4};
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::imgui;
use imgui_glfw_rs::ImguiGLFW;
use imgui_opengl_renderer::Renderer;

pub struct AppUI {
    imgui: ImGui,
    imgui_glfw: ImguiGLFW,
    imgui_renderer: Renderer,

    pub is_ingame: bool,
    is_ingame_menu: bool,
    pub is_menu_control: bool,

    is_key_esc: bool,
}

impl AppUI {
    pub fn new(window: &mut Window) -> Self {
        let mut imgui = AppUI::init_imgui();

        let imgui_glfw = ImguiGLFW::new(&mut imgui);

        let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| window.get_proc_address(s) as _);

        Self {
            imgui,
            imgui_glfw,
            imgui_renderer,

            is_ingame: false,
            is_ingame_menu: false,
            is_menu_control: false,

            is_key_esc: false,
        }
    }

    pub fn init_imgui() -> ImGui {
        let mut imgui = ImGui::init();

        {
            let style = imgui.style_mut();

            style.child_rounding = 3.0;
            style.grab_rounding = 3.0;
            style.window_rounding = 6.0;
            style.scrollbar_rounding = 3.0;
            style.frame_rounding = 3.0;
            style.window_title_align = ImVec2::new(0.5, 0.5);

            style.colors[ImGuiCol::Text as usize] = ImVec4::new(0.93, 0.93, 0.93, 1.00);
            style.colors[ImGuiCol::TextDisabled as usize] = ImVec4::new(0.70, 0.70, 0.70, 1.00);
            style.colors[ImGuiCol::WindowBg as usize] = ImVec4::new(0.16, 0.16, 0.16, 0.70);
            style.colors[ImGuiCol::ChildBg as usize] = ImVec4::new(0.18, 0.18, 0.18, 0.70);
            style.colors[ImGuiCol::PopupBg as usize] = ImVec4::new(0.26, 0.26, 0.26, 0.70);
            style.colors[ImGuiCol::Border as usize] = ImVec4::new(0.26, 0.26, 0.26, 1.00);
            style.colors[ImGuiCol::BorderShadow as usize] = ImVec4::new(0.26, 0.26, 0.26, 1.00);
            style.colors[ImGuiCol::FrameBg as usize] = ImVec4::new(0.16, 0.16, 0.16, 0.40);
            style.colors[ImGuiCol::FrameBgHovered as usize] = ImVec4::new(0.16, 0.16, 0.16, 0.60);
            style.colors[ImGuiCol::FrameBgActive as usize] = ImVec4::new(0.16, 0.16, 0.16, 0.60);
            style.colors[ImGuiCol::TitleBg as usize] = ImVec4::new(0.36, 0.36, 0.36, 0.80);
            style.colors[ImGuiCol::TitleBgCollapsed as usize] = ImVec4::new(0.36, 0.36, 0.36, 0.80);
            style.colors[ImGuiCol::TitleBgActive as usize] = ImVec4::new(0.36, 0.36, 0.36, 0.80);
            style.colors[ImGuiCol::MenuBarBg as usize] = ImVec4::new(0.26, 0.26, 0.26, 1.00);
            style.colors[ImGuiCol::ScrollbarBg as usize] = ImVec4::new(0.21, 0.21, 0.21, 1.00);
            style.colors[ImGuiCol::ScrollbarGrab as usize] = ImVec4::new(0.36, 0.36, 0.36, 1.00);
            style.colors[ImGuiCol::ScrollbarGrabHovered as usize] = ImVec4::new(0.36, 0.36, 0.36, 1.00);
            style.colors[ImGuiCol::ScrollbarGrabActive as usize] = ImVec4::new(0.36, 0.36, 0.36, 1.00);
            style.colors[ImGuiCol::CheckMark as usize] = ImVec4::new(0.78, 0.78, 0.78, 1.00);
            style.colors[ImGuiCol::SliderGrab as usize] = ImVec4::new(0.74, 0.74, 0.74, 1.00);
            style.colors[ImGuiCol::SliderGrabActive as usize] = ImVec4::new(0.74, 0.74, 0.74, 1.00);
            style.colors[ImGuiCol::Button as usize] = ImVec4::new(0.36, 0.36, 0.36, 1.00);
            style.colors[ImGuiCol::ButtonHovered as usize] = ImVec4::new(0.43, 0.43, 0.43, 1.00);
            style.colors[ImGuiCol::ButtonActive as usize] = ImVec4::new(0.11, 0.11, 0.11, 1.00);
            style.colors[ImGuiCol::Header as usize] = ImVec4::new(0.36, 0.36, 0.36, 1.00);
            style.colors[ImGuiCol::HeaderHovered as usize] = ImVec4::new(0.36, 0.36, 0.36, 1.00);
            style.colors[ImGuiCol::HeaderActive as usize] = ImVec4::new(0.36, 0.36, 0.36, 1.00);
            style.colors[ImGuiCol::ResizeGrip as usize] = ImVec4::new(0.36, 0.36, 0.36, 1.00);
            style.colors[ImGuiCol::ResizeGripHovered as usize] = ImVec4::new(0.6, 0.4, 0.0, 1.0);
            style.colors[ImGuiCol::ResizeGripActive as usize] = ImVec4::new(0.7, 0.5, 0.0, 1.0);
            style.colors[ImGuiCol::PlotLines as usize] = ImVec4::new(0.39, 0.39, 0.39, 1.00);
            style.colors[ImGuiCol::PlotLinesHovered as usize] = ImVec4::new(1.00, 0.43, 0.35, 1.00);
            style.colors[ImGuiCol::PlotHistogram as usize] = ImVec4::new(0.10, 0.70, 0.00, 1.00);
            style.colors[ImGuiCol::PlotHistogramHovered as usize] = ImVec4::new(1.00, 0.60, 0.00, 1.00);
            style.colors[ImGuiCol::TextSelectedBg as usize] = ImVec4::new(0.32, 0.52, 0.65, 1.00);
            style.colors[ImGuiCol::ModalWindowDimBg as usize] = ImVec4::new(0.20, 0.20, 0.20, 0.50);
        }

        imgui.set_ini_filename(None);

        let font_size = 22.0 as f32;

        imgui
            .fonts()
            .add_default_font_with_config(ImFontConfig::new().oversample_h(1).pixel_snap_h(true).size_pixels(font_size));

        imgui.fonts().add_font_with_config(
            include_bytes!("../../res/fonts/ProFontWindows.ttf"),
            ImFontConfig::new()
                .merge_mode(true)
                .oversample_h(1)
                .pixel_snap_h(true)
                .size_pixels(font_size)
                .rasterizer_multiply(1.75),
            &FontGlyphRange::default(),
        );

        imgui.set_font_global_scale(1.0);

        imgui
    }

    pub fn draw(&mut self, window: &mut Window, scene: &mut Scene, settings: &mut GameSettings) {
        let ui = self.imgui_glfw.frame(window, &mut self.imgui);

        if self.is_ingame {
            draw_game_ui(window, scene, settings, &ui, &mut self.is_ingame_menu, &mut self.is_key_esc, &mut self.is_ingame);
        }
        else {
            draw_main_menu(window, scene, settings, &ui, &mut self.is_key_esc, &mut self.is_ingame);
        }

        let is_menu_changed = self.is_menu_control;
        self.is_menu_control = self.is_ingame_menu || !self.is_ingame;
        if self.is_menu_control != is_menu_changed {
            let (win_width, win_height) = window.get_size();
            let curs_x = win_width / 2;
            let curs_y = win_height / 2;
            window.set_cursor_pos(curs_x as f64, curs_y as f64);
            if self.is_menu_control {
                window.set_cursor_mode(glfw::CursorMode::Normal);
            } else {
                window.set_cursor_mode(glfw::CursorMode::Disabled);
            }
        }

        self.imgui_renderer.render(ui);
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        self.imgui_glfw.handle_event(&mut self.imgui, event);
    }
}
