mod event_loop;
mod tiny_renderer;

use conrod::backend::glium::glium::{self, Surface};
use conrod::{self, *};

pub fn main() {
    const WIDTH: u32 = 500;
    const HEIGHT: u32 = 500;

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Image Widget Demonstration")
        .with_dimensions((WIDTH, HEIGHT).into());
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
    widget_ids!(struct Ids { background, image_widget });
    let ids = Ids::new(ui.widget_id_generator());

    // Create our `conrod::image::Map` which describes each of our widget->image mappings.
    const IMAGE_SIZE: (u32, u32) = (500, 500);
    let mut rgb_image = tiny_renderer::Image::new(IMAGE_SIZE.0 as usize, IMAGE_SIZE.1 as usize);

    let raw_image = glium::texture::RawImage2d::from_raw_rgb(rgb_image.data().to_vec(), IMAGE_SIZE);
    let texture = glium::texture::Texture2d::new(&display, raw_image).unwrap();
    let (w, h) = (texture.get_width(), texture.get_height().unwrap());

    let mut image_map = conrod::image::Map::new();
    let image_widget = image_map.insert(texture);

    // Poll events from the window.
    let mut event_loop = event_loop::EventLoop::new();
    'main: loop {
        // Handle all events.
        for event in event_loop.next(&mut events_loop) {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::CloseRequested
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

        // Update texture
        let updated = tiny_renderer::update(&mut rgb_image);
        if updated {
            let raw_image =
                glium::texture::RawImage2d::from_raw_rgb(rgb_image.data().to_vec(), IMAGE_SIZE);
            let texture = glium::texture::Texture2d::new(&display, raw_image).unwrap();
            image_map.replace(image_widget, texture);
        }

        // Instantiate the widgets.
        {
            let ui = &mut ui.set_widgets();
            // Draw a light blue background.
            widget::Canvas::new()
                .color(color::LIGHT_BLUE)
                .set(ids.background, ui);
            // Instantiate the `Image` at its full size in the middle of the window.
            widget::Image::new(image_widget)
                .w_h(w as f64, h as f64)
                .middle()
                .set(ids.image_widget, ui);
        }

        // Render the `Ui` and then display it on the screen.
        let primitives = ui.draw();
        renderer.fill(&display, primitives, &image_map);
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        renderer.draw(&display, &mut target, &image_map).unwrap();
        target.finish().unwrap();
    }
}
