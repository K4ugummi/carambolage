use crate::game::scene::Scene;
use gl;
use glfw::{Window, WindowEvent};
use imgui::{im_str, FontGlyphRange, ImFontConfig, ImGui, ImGuiCol, ImVec2, ImVec4};
use imgui_glfw_rs::ImguiGLFW;
use imgui_opengl_renderer::Renderer;

pub struct AppUI {
    imgui: ImGui,
    imgui_glfw: ImguiGLFW,
    imgui_renderer: Renderer,

    pub is_ingame: bool,
    pub is_ingame_menu: bool,

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

            is_ingame: true,
            is_ingame_menu: false,

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

            style.colors[ImGuiCol::Text as usize] = ImVec4::new(0.13, 0.13, 0.13, 1.00);
            style.colors[ImGuiCol::TextDisabled as usize] = ImVec4::new(0.50, 0.50, 0.50, 1.00);
            style.colors[ImGuiCol::WindowBg as usize] = ImVec4::new(0.16, 0.16, 0.16, 0.60);
            style.colors[ImGuiCol::ChildBg as usize] = ImVec4::new(0.18, 0.18, 0.18, 0.60);
            style.colors[ImGuiCol::PopupBg as usize] = ImVec4::new(0.26, 0.26, 0.26, 0.60);
            style.colors[ImGuiCol::Border as usize] = ImVec4::new(0.26, 0.26, 0.26, 1.00);
            style.colors[ImGuiCol::BorderShadow as usize] = ImVec4::new(0.26, 0.26, 0.26, 1.00);
            style.colors[ImGuiCol::FrameBg as usize] = ImVec4::new(0.96, 0.16, 0.16, 0.60);
            style.colors[ImGuiCol::FrameBgHovered as usize] = ImVec4::new(0.16, 0.16, 0.16, 0.60);
            style.colors[ImGuiCol::FrameBgActive as usize] = ImVec4::new(0.16, 0.16, 0.16, 0.60);
            style.colors[ImGuiCol::TitleBg as usize] = ImVec4::new(0.36, 0.36, 0.36, 0.60);
            style.colors[ImGuiCol::TitleBgCollapsed as usize] = ImVec4::new(0.36, 0.36, 0.36, 0.60);
            style.colors[ImGuiCol::TitleBgActive as usize] = ImVec4::new(0.36, 0.36, 0.36, 0.60);
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
            style.colors[ImGuiCol::CloseButton as usize] = ImVec4::new(0.59, 0.59, 0.59, 1.00);
            style.colors[ImGuiCol::CloseButtonHovered as usize] = ImVec4::new(0.98, 0.39, 0.36, 1.00);
            style.colors[ImGuiCol::CloseButtonActive as usize] = ImVec4::new(0.98, 0.39, 0.36, 1.00);
            style.colors[ImGuiCol::PlotLines as usize] = ImVec4::new(0.39, 0.39, 0.39, 1.00);
            style.colors[ImGuiCol::PlotLinesHovered as usize] = ImVec4::new(1.00, 0.43, 0.35, 1.00);
            style.colors[ImGuiCol::PlotHistogram as usize] = ImVec4::new(0.10, 0.70, 0.00, 1.00);
            style.colors[ImGuiCol::PlotHistogramHovered as usize] = ImVec4::new(1.00, 0.60, 0.00, 1.00);
            style.colors[ImGuiCol::TextSelectedBg as usize] = ImVec4::new(0.32, 0.52, 0.65, 1.00);
            style.colors[ImGuiCol::ModalWindowDarkening as usize] = ImVec4::new(0.20, 0.20, 0.20, 0.50);
        }

        imgui.set_ini_filename(None);

        let font_size = 22.0 as f32;

        imgui
            .fonts()
            .add_default_font_with_config(ImFontConfig::new().oversample_h(1).pixel_snap_h(true).size_pixels(font_size));

        imgui.fonts().add_font_with_config(
            include_bytes!("../../res/fonts/SansForgetica-Regular.otf"),
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

    pub fn draw(&mut self, window: &mut Window, scene: &mut Scene) {
        let ui = self.imgui_glfw.frame(window, &mut self.imgui);

        let (width, height) = window.get_size();
        let width = width as f32;
        let height = height as f32;

        let mut should_close = false;

        if self.is_ingame {
            ui.window(im_str!("Player 1"))
                .title_bar(true)
                .position((20.0, height - 100.), imgui::ImGuiCond::Always)
                .size((250.0, 0.0), imgui::ImGuiCond::Once)
                .always_use_window_padding(true)
                .collapsible(false)
                .resizable(false)
                .movable(false)
                .build(|| {
                    ui.progress_bar(scene.cars[0].boost / 100.)
                        .overlay_text(im_str!("BOOST"))
                        .size((-1., 40.))
                        .build();
                });
            ui.window(im_str!("Player 2"))
                .title_bar(true)
                .position((width - 270., height - 100.), imgui::ImGuiCond::Always)
                .size((250.0, 0.0), imgui::ImGuiCond::Once)
                .always_use_window_padding(true)
                .collapsible(false)
                .resizable(false)
                .movable(false)
                .build(|| {
                    ui.progress_bar(scene.cars[1].boost / 100.)
                        .overlay_text(im_str!("BOOST"))
                        .size((-1., 40.))
                        .build();
                });

            if !self.is_key_esc && window.get_key(glfw::Key::Escape) == glfw::Action::Press {
                if self.is_ingame_menu {
                    ui.close_current_popup();
                }
                self.is_ingame_menu = !self.is_ingame_menu;
                self.is_key_esc = true;
            } else if self.is_key_esc && window.get_key(glfw::Key::Escape) == glfw::Action::Release {
                self.is_key_esc = false;
            }

            if self.is_ingame_menu {
                ui.open_popup(im_str!("Menu"));
            }

            ui.popup_modal(im_str!("Menu"))
                .title_bar(false)
                //.position((width / 2. - 100., height / 2.), imgui::ImGuiCond::Always)
                //.size((200., 0.), imgui::ImGuiCond::Always)
                .always_use_window_padding(true)
                .collapsible(false)
                .resizable(false)
                .movable(false)
                .build(|| {
                    if ui.button(im_str!("Continue"), (200., 40.)) {
                        ui.close_current_popup();
                    }
                    ui.separator();
                    if ui.button(im_str!("Exit"), (200., 40.)) {
                        should_close = true;
                    }
                })
        }

        window.set_should_close(should_close);

        self.imgui_renderer.render(ui);
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        self.imgui_glfw.handle_event(&mut self.imgui, event);
    }
}
