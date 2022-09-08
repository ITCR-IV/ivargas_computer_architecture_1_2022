use anyhow::Result;
use image::{
    imageops::{colorops::grayscale, resize, FilterType},
    io::Reader as ImageReader,
    GrayImage,
};
use softbuffer::GraphicsContext;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

mod interpolation;
mod utils;

const IMAGE: &str = "input.jpg";
const N: u32 = 4;

fn main() -> Result<()> {
    let buf: [u8; 3] = [1, 2, 3];
    let sum = interpolation::bil_interpol("result.img", &buf, 1, 3);
    println!("ASM return: {}", sum);

    let gray_img: GrayImage = grayscale(&ImageReader::open(IMAGE)?.decode()?);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(window) }.unwrap();

    let mut mouse_pos = PhysicalPosition::<f64>::new(0.0, 0.0);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();
        let (width, height) = {
            let size = graphics_context.window().inner_size();
            (size.width, size.height)
        };

        match event {
            Event::RedrawRequested(window_id) if window_id == graphics_context.window().id() => {
                let res_img = resize(&gray_img, width, height, FilterType::Nearest);

                let buffer: Vec<u32> = utils::gray_to_vec32(res_img);
                let buffer = utils::draw_grid(buffer, width, height, N).unwrap();

                graphics_context.set_buffer(&buffer, width as u16, height as u16);
            }

            Event::WindowEvent { event, window_id }
                if window_id == graphics_context.window().id() =>
            {
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::CursorMoved { position, .. } => {
                        mouse_pos = position;
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        if state == ElementState::Pressed && button == MouseButton::Left {
                            println!("{:?}", mouse_pos);
                            println!(
                                "Grid Square: {}",
                                utils::check_grid(mouse_pos.x, mouse_pos.y, width, height, N)
                            );
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });


}
