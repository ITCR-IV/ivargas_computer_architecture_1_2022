use anyhow::Result;
use image::{
    imageops::{colorops::grayscale, resize, FilterType},
    io::Reader as ImageReader,
    GrayImage, ImageBuffer,
};
use softbuffer::GraphicsContext;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

mod interpolation;
mod utils;

const IMAGE: &str = "input.jpg";
const RESULT: &str = "result.img";
const N: u32 = 4;

enum ImageState {
    BeforeSelection(GrayImage),
    AfterSelection(GrayImage),
}

fn main() -> Result<()> {
    let gray_img: GrayImage = grayscale(&ImageReader::open(IMAGE)?.decode()?);
    gray_img.save("input.png").unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut graphics_context: GraphicsContext<Window> =
        unsafe { GraphicsContext::new(window) }.unwrap();

    let mut mouse_pos = PhysicalPosition::<f64>::new(0.0, 0.0);

    let mut img_state: ImageState = ImageState::BeforeSelection(gray_img);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        let (width, height) = {
            let size = graphics_context.window().inner_size();
            (size.width, size.height)
        };

        match event {
            Event::RedrawRequested(window_id) if window_id == graphics_context.window().id() => {
                handle_redraw(&mut graphics_context, &img_state, height, width);
            }

            Event::WindowEvent { event, window_id }
                if window_id == graphics_context.window().id() =>
            {
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::CursorMoved { position, .. } => {
                        mouse_pos = position;
                    }
                    WindowEvent::MouseInput { state, button, .. } => handle_input(
                        &graphics_context,
                        &mut img_state,
                        mouse_pos,
                        height,
                        width,
                        state,
                        button,
                    ),
                    _ => {}
                }
            }
            _ => {}
        }
    });
}

fn handle_redraw(
    graphics_context: &mut GraphicsContext<Window>,
    img_state: &ImageState,
    height: u32,
    width: u32,
) {
    match &img_state {
        ImageState::BeforeSelection(img) => {
            let res_img = resize(img, width, height, FilterType::Nearest);

            let buffer: Vec<u32> = utils::gray_to_vec32(res_img);
            let buffer = utils::draw_grid(buffer, width, height, N).unwrap();

            graphics_context.set_buffer(&buffer, width as u16, height as u16);
        }
        ImageState::AfterSelection(img) => {
            let res_img = resize(img, width, height, FilterType::Nearest);

            let buffer: Vec<u32> = utils::gray_to_vec32(res_img);
            graphics_context.set_buffer(&buffer, width as u16, height as u16);
        }
    }
}

fn handle_input(
    graphics_context: &GraphicsContext<Window>,
    img_state: &mut ImageState,
    mouse_pos: PhysicalPosition<f64>,
    height: u32,
    width: u32,
    state: ElementState,
    button: MouseButton,
) {
    if state == ElementState::Pressed && button == MouseButton::Left {
        if let ImageState::BeforeSelection(img) = &img_state {
            let square: u32 = utils::check_grid(mouse_pos.x, mouse_pos.y, width, height, N);
            let square_width: f64 = img.width() as f64 / N as f64;
            let square_height: f64 = img.height() as f64 / N as f64;

            let x: u32 = ((square % N) as f64 * square_width).round() as u32;
            let y: u32 = ((square / N) as f64 * square_height).round() as u32;
            let cut_image =
                utils::cut_image(img, x, y, x + square_width as u32, y + square_width as u32);

            interpolation::bil_interpol(
                RESULT,
                cut_image.as_raw(),
                cut_image.height(),
                cut_image.width(),
            );

            let interpolated_buffer = std::fs::read(RESULT).unwrap();

            let interpolated_image: GrayImage = ImageBuffer::from_vec(
                cut_image.width() * 3 - 2,
                cut_image.height() * 3 - 2,
                interpolated_buffer,
            )
            .unwrap();

            interpolated_image.save("result.png").unwrap();

            *img_state = ImageState::AfterSelection(interpolated_image);

            graphics_context.window().request_redraw();
        }
    }
}
