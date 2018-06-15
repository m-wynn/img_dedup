use config::Config;
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;
use conrod::{self, color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
use image;
use img_hash::HashType;
use std;

mod eventloop;

pub fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Image Deduplicator")
        .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The `WidgetId` for our background and `Image` widgets.
    widget_ids!(struct Ids { background, directory, method, rust_logo, submit });
    let ids = Ids::new(ui.widget_id_generator());

    let font_path = "./assets/fonts/NotoSans-Regular.ttf";
    ui.fonts.insert_from_file(font_path).unwrap();

    // Create our `conrod::image::Map` which describes each of our widget->image mappings.
    // In our case we only have one image, however the macro may be used to list multiple.
    let rust_logo = load_rust_logo(&display);
    let (w, h) = (rust_logo.get_width(), rust_logo.get_height().unwrap());
    let mut image_map = conrod::image::Map::new();
    let rust_logo = image_map.insert(rust_logo);

    let mut sdirectory = "./".to_owned();
    let method = vec!["mean", "block", "doublegradient", "dct", "gradient"];
    let mut selected_method_index = 4;
    // Poll events from the window.
    let mut event_loop = eventloop::EventLoop::new();
    'main: loop {
        // Handle all events.
        for event in event_loop.next(&mut events_loop) {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
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

        // Instantiate the widgets.
        {
            let ui = &mut ui.set_widgets();
            // Draw a light blue background.
            widget::Canvas::new()
                .color(color::LIGHT_BLUE)
                .set(ids.background, ui);
            // Instantiate the `Image` at its full size in the middle of the window.
            // widget::Image::new(rust_logo)
            //     .w_h(w as f64, h as f64)
            //     .middle()
            //     .set(ids.rust_logo, ui);

            for event in widget::TextBox::new(&sdirectory)
                .color(color::WHITE)
                .text_color(color::BLACK)
                .font_size(20)
                .w_h(320.0, 40.0)
                .middle()
                .set(ids.directory, ui)
            {
                match event {
                    widget::text_box::Event::Enter => println!("TextBox: {:?}", sdirectory),
                    widget::text_box::Event::Update(string) => sdirectory = string,
                }
            }

            for i in widget::DropDownList::new(&method, Some(selected_method_index))
                .color(color::WHITE)
                .w_h(320.0, 40.0)
                .mid_top_of(ids.background)
                .set(ids.method, ui)
            {
                selected_method_index = i;
            }

            if widget::Button::new()
                .label("Deduplicate!")
                .w_h(320.0, 40.0)
                .mid_bottom_of(ids.background)
                .set(ids.submit, ui)
                .was_clicked()
            {
                println!("Submit");
            }
        }
        display
            .gl_window()
            .window()
            .set_cursor(conrod::backend::winit::convert_mouse_cursor(
                ui.mouse_cursor(),
            ));

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

// Load the Rust logo from our assets folder to use as an example image.
fn load_rust_logo(display: &glium::Display) -> glium::texture::Texture2d {
    let path = "./test/rustB250.jpg";
    let rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
    let image_dimensions = rgba_image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
        &rgba_image.into_raw(),
        image_dimensions,
    );
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}
