mod game_ui;
mod main_menu_ui;

use self::{game_ui::*, main_menu_ui::*};
use crate::game::GameSettings;
use crate::game::scene::Scene;
use glfw::{Window, WindowEvent};
use imgui::{Context as ImContext, FontConfig, StyleColor};
use imgui_glfw_rs::ImguiGLFW;
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::imgui;
use imgui_opengl_renderer_rs::Renderer;

pub struct AppUI {
    imgui: ImContext,
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

        let imgui_glfw = ImguiGLFW::new(&mut imgui, window);

        let imgui_renderer = imgui_opengl_renderer_rs::Renderer::new(&mut imgui, |s| window.get_proc_address(s) as _);

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

    pub fn init_imgui() -> ImContext {
        let mut imgui = ImContext::create();

        {
            let style = imgui.style_mut();

            style.child_rounding = 3.0;
            style.grab_rounding = 3.0;
            style.window_rounding = 6.0;
            style.scrollbar_rounding = 3.0;
            style.frame_rounding = 3.0;
            style.window_title_align = [0.5, 0.5];

            style.colors[StyleColor::Text as usize] = [0.93, 0.93, 0.93, 1.00];
            style.colors[StyleColor::TextDisabled as usize] = [0.70, 0.70, 0.70, 1.00];
            style.colors[StyleColor::WindowBg as usize] = [0.16, 0.16, 0.16, 0.70];
            style.colors[StyleColor::ChildBg as usize] = [0.18, 0.18, 0.18, 0.70];
            style.colors[StyleColor::PopupBg as usize] = [0.26, 0.26, 0.26, 0.70];
            style.colors[StyleColor::Border as usize] = [0.26, 0.26, 0.26, 1.00];
            style.colors[StyleColor::BorderShadow as usize] = [0.26, 0.26, 0.26, 1.00];
            style.colors[StyleColor::FrameBg as usize] = [0.16, 0.16, 0.16, 0.40];
            style.colors[StyleColor::FrameBgHovered as usize] = [0.16, 0.16, 0.16, 0.60];
            style.colors[StyleColor::FrameBgActive as usize] = [0.16, 0.16, 0.16, 0.60];
            style.colors[StyleColor::TitleBg as usize] = [0.36, 0.36, 0.36, 0.80];
            style.colors[StyleColor::TitleBgCollapsed as usize] = [0.36, 0.36, 0.36, 0.80];
            style.colors[StyleColor::TitleBgActive as usize] = [0.36, 0.36, 0.36, 0.80];
            style.colors[StyleColor::MenuBarBg as usize] = [0.26, 0.26, 0.26, 1.00];
            style.colors[StyleColor::ScrollbarBg as usize] = [0.21, 0.21, 0.21, 1.00];
            style.colors[StyleColor::ScrollbarGrab as usize] = [0.36, 0.36, 0.36, 1.00];
            style.colors[StyleColor::ScrollbarGrabHovered as usize] = [0.36, 0.36, 0.36, 1.00];
            style.colors[StyleColor::ScrollbarGrabActive as usize] = [0.36, 0.36, 0.36, 1.00];
            style.colors[StyleColor::CheckMark as usize] = [0.78, 0.78, 0.78, 1.00];
            style.colors[StyleColor::SliderGrab as usize] = [0.74, 0.74, 0.74, 1.00];
            style.colors[StyleColor::SliderGrabActive as usize] = [0.74, 0.74, 0.74, 1.00];
            style.colors[StyleColor::Button as usize] = [0.36, 0.36, 0.36, 1.00];
            style.colors[StyleColor::ButtonHovered as usize] = [0.43, 0.43, 0.43, 1.00];
            style.colors[StyleColor::ButtonActive as usize] = [0.11, 0.11, 0.11, 1.00];
            style.colors[StyleColor::Header as usize] = [0.36, 0.36, 0.36, 1.00];
            style.colors[StyleColor::HeaderHovered as usize] = [0.36, 0.36, 0.36, 1.00];
            style.colors[StyleColor::HeaderActive as usize] = [0.36, 0.36, 0.36, 1.00];
            style.colors[StyleColor::ResizeGrip as usize] = [0.36, 0.36, 0.36, 1.00];
            style.colors[StyleColor::ResizeGripHovered as usize] = [0.6, 0.4, 0.0, 1.0];
            style.colors[StyleColor::ResizeGripActive as usize] = [0.7, 0.5, 0.0, 1.0];
            style.colors[StyleColor::PlotLines as usize] = [0.39, 0.39, 0.39, 1.00];
            style.colors[StyleColor::PlotLinesHovered as usize] = [1.00, 0.43, 0.35, 1.00];
            style.colors[StyleColor::PlotHistogram as usize] = [0.10, 0.70, 0.00, 1.00];
            style.colors[StyleColor::PlotHistogramHovered as usize] = [1.00, 0.60, 0.00, 1.00];
            style.colors[StyleColor::TextSelectedBg as usize] = [0.32, 0.52, 0.65, 1.00];
            style.colors[StyleColor::ModalWindowDimBg as usize] = [0.20, 0.20, 0.20, 0.50];
        }

        imgui.set_ini_filename(None);

        let font_size = 22.0 as f32;

        imgui.fonts().add_font(&[imgui::FontSource::DefaultFontData { config: None }]);
        // .add_default_font_with_config(FontConfig::new().oversample_h(1).pixel_snap_h(true).size_pixels(font_size));

        // imgui.fonts().add_font_with_config(
        //     include_bytes!("../../res/fonts/ProFontWindows.ttf"),
        //     FontConfig::new()
        //         .merge_mode(true)
        //         .oversample_h(1)
        //         .pixel_snap_h(true)
        //         .size_pixels(font_size)
        //         .rasterizer_multiply(1.75),
        //     &FontGlyphRange::default(),
        // );

        // imgui.set_font_global_scale(1.0);

        imgui
    }

    pub fn draw(&mut self, window: &mut Window, scene: &mut Scene, settings: &mut GameSettings) {
        let framerate = self.imgui.io_mut().framerate;
        let ui = self.imgui_glfw.frame(window, &mut self.imgui);
        if self.is_ingame {
            draw_game_ui(
                window,
                scene,
                settings,
                &ui,
                &mut self.is_ingame_menu,
                &mut self.is_key_esc,
                &mut self.is_ingame,
                framerate,
            );
        } else {
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

        self.imgui_renderer.render(&mut self.imgui);
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        self.imgui_glfw.handle_event(&mut self.imgui, event);
    }
}
