use config::Config;
use conrod;
use conrod::backend::glium::glium::{self, Surface};
use conrod::text::Font;

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
        spinner
    }
}

pub trait WindowContents {
    fn set_ui(
        &mut self,
        display: &glium::Display,
        image_map: &mut conrod::image::Map<glium::Texture2d>,
        ui: &mut conrod::UiCell,
        ids: &Ids,
        config: &mut Config,
    ) -> Option<Box<WindowContents>>;
}

pub fn main(mut config: Config) {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Image Deduplicator")
        .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([f64::from(WIDTH), f64::from(HEIGHT)]).build();

    let ids = Ids::new(ui.widget_id_generator());

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    ui.fonts.insert(Font::from_bytes(include_bytes!("assets/fonts/NotoSans-Regular.ttf").to_vec()).unwrap());

    let mut current_window: Box<WindowContents> =
        Box::new(configwindow::ConfigWindow::new(&config));
    // Poll events from the window.
    let mut event_loop = eventloop::EventLoop::new();
    'main: loop {
        for event in event_loop.next(&mut events_loop) {
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::WindowEvent::Closed
                    | glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        if let Some(new_window) = current_window.set_ui(&display, &mut image_map, &mut ui.set_widgets(), &ids, &mut config) {
            current_window = new_window;
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
