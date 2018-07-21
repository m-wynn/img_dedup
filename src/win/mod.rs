use conrod;
use conrod::backend::{
    glium::{
        glium::{
            self,
            glutin::{
                dpi::LogicalSize, ContextBuilder, Event, EventsLoop, KeyboardInput, VirtualKeyCode,
                WindowBuilder, WindowEvent,
            },
            texture, Display, Surface,
        },
        Renderer,
    },
    winit,
};
use conrod::text::Font;
use conrod::UiCell;
use conrod::widget_ids;
use crate::config::Config;

mod comparewindow;
mod configwindow;
mod eventloop;
mod waitwindow;

widget_ids!{
    pub struct Ids {
        background,
        directory,
        method,
        submit,
        waiting,
    }
}

pub struct WindowContext<'a> {
    display: &'a glium::Display,
    image_map: &'a mut conrod::image::Map<glium::Texture2d>,
    ids: &'a Ids,
    config: &'a mut Config,
}

pub trait WindowContents {
    fn set_ui(&mut self, win_ctx: &mut WindowContext, ui: &mut UiCell) -> Option<Box<WindowContents>>;
}

pub fn main(mut config: Config) {
    const WIDTH: f64 = 800.0;
    const HEIGHT: f64 = 600.0;

    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_title("Image Deduplicator")
        .with_dimensions(LogicalSize::new(WIDTH, HEIGHT));
    let context = ContextBuilder::new().with_vsync(true).with_multisampling(4);
    let display = Display::new(window, context, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH, HEIGHT]).build();

    let ids = Ids::new(ui.widget_id_generator());

    let mut renderer = Renderer::new(&display).unwrap();

    let mut image_map = conrod::image::Map::<texture::Texture2d>::new();

    ui.fonts.insert(
        Font::from_bytes(include_bytes!("assets/fonts/NotoSans-Regular.ttf").to_vec()).unwrap(),
    );

    let mut current_window: Box<WindowContents> =
        Box::new(configwindow::ConfigWindow::new(&config));
    // Poll events from the window.
    let mut event_loop = eventloop::EventLoop::new();

    'main: loop {
        for event in event_loop.next(&mut events_loop) {
            if let Some(event) = winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::Destroyed
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        {
            let mut win_ctx = WindowContext {
                display: &display,
                image_map: &mut image_map,
                ids: &ids,
                config: &mut config,
            };

            if let Some(new_window) = current_window.set_ui(&mut win_ctx, &mut ui.set_widgets()) {
                current_window = new_window;
            }
        }

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}
