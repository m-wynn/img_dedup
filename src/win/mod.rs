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
use conrod::{self, text::Font, widget_ids, Ui, UiCell};
use failure::Error;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

pub mod comparewindow;
pub mod configwindow;
mod eventloop;
pub mod waitwindow;

widget_ids!{
    pub struct Ids {
        background,
        directory,
        method,
        submit,
        waiting,
    }
}

pub struct Win {
    current_window: Arc<Mutex<Box<WindowContents>>>,
    display: glium::Display,
    event_loop: eventloop::EventLoop,
    events_loop: EventsLoop,
    ids: Ids,
    image_map: conrod::image::Map<glium::Texture2d>,
    renderer: Renderer,
    ui: Ui,
}

pub trait WindowContents : Send {
    fn set_ui(&mut self, ui: &mut UiCell, ids: &mut Ids);
}

impl Win {
    pub fn new(window_contents: Arc<Mutex<Box<WindowContents>>>) -> Result<Win, Error> {
        const WIDTH: f64 = 800.0;
        const HEIGHT: f64 = 600.0;

        let mut events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_title("Image Deduplicator")
            .with_dimensions(LogicalSize::new(WIDTH, HEIGHT));
        let context = ContextBuilder::new().with_vsync(true).with_multisampling(4);
        let display = Display::new(window, context, &events_loop).unwrap(); // todo: Handle this unwrap dude

        let mut ui = conrod::UiBuilder::new([WIDTH, HEIGHT]).build();

        let ids = Ids::new(ui.widget_id_generator());

        let mut renderer = Renderer::new(&display)?;

        let mut image_map = conrod::image::Map::<texture::Texture2d>::new();

        ui.fonts.insert(
            Font::from_bytes(include_bytes!("assets/fonts/NotoSans-Regular.ttf").to_vec())?,
        );

        // Poll events from the window.
        let mut event_loop = eventloop::EventLoop::new();

        Ok(Win {
            current_window: window_contents,
            display: display,
            event_loop: event_loop,
            events_loop: events_loop,
            ids: ids,
            image_map: image_map,
            renderer: renderer,
            ui: ui,
        })
    }

    pub fn update(&mut self) {
        'main: loop {
            for event in self.event_loop.next(&mut self.events_loop) {
                if let Some(event) = winit::convert_event(event.clone(), &self.display) {
                    self.ui.handle_event(event);
                    self.event_loop.needs_update();
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

            self.current_window.lock().unwrap().set_ui(&mut self.ui.set_widgets(), &mut self.ids);

            if let Some(primitives) = self.ui.draw_if_changed() {
                self.renderer.fill(&self.display, primitives, &self.image_map);
                let mut target = self.display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }
}
