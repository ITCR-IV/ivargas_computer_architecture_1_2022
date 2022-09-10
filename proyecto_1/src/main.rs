use anyhow::Result;
use clap::Parser;
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

#[derive(Parser, Clone, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input image to use
    #[clap(short)]
    input: String,

    /// Save grayscale version of input image
    #[clap(short)]
    gray: Option<String>,

    /// Interpolated section output if not specified won't be saved
    #[clap(short)]
    output: Option<String>,

    #[clap(short = 'm', default_value = "result.img")]
    intermediate: String,

    /// divide into n^2 squares
    #[clap(short, default_value_t = 4)]
    n: u32,
}

enum ImageState {
    BeforeSelection(GrayImage),
    AfterSelection(GrayImage),
}

struct Context {
    args: Args,
    graphics_context: GraphicsContext<Window>,
    img_state: ImageState,
    mouse_pos: PhysicalPosition<f64>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let gray_img: GrayImage = grayscale(&ImageReader::open(&args.input)?.decode()?);
    gray_img.save("input.png").unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut context = Context {
        args,
        graphics_context: unsafe { GraphicsContext::new(window) }.unwrap(),
        img_state: ImageState::BeforeSelection(gray_img),
        mouse_pos: PhysicalPosition::<f64>::new(0.0, 0.0),
    };

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        let (win_width, win_height) = {
            let size = context.graphics_context.window().inner_size();
            (size.width, size.height)
        };

        match event {
            Event::RedrawRequested(window_id)
                if window_id == context.graphics_context.window().id() =>
            {
                handle_redraw(&mut context, win_height, win_width);
            }

            Event::WindowEvent { event, window_id }
                if window_id == context.graphics_context.window().id() =>
            {
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::CursorMoved { position, .. } => {
                        context.mouse_pos = position;
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        handle_input(&mut context, win_height, win_width, state, button)
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}

fn handle_redraw(context: &mut Context, win_height: u32, win_width: u32) {
    match &context.img_state {
        ImageState::BeforeSelection(img) => {
            let res_img = resize(img, win_width, win_height, FilterType::Nearest);

            let buffer: Vec<u32> = utils::gray_to_vec32(res_img);
            let buffer = utils::draw_grid(buffer, win_width, win_height, context.args.n).unwrap();

            context
                .graphics_context
                .set_buffer(&buffer, win_width as u16, win_height as u16);
        }
        ImageState::AfterSelection(img) => {
            let res_img = resize(img, win_width, win_height, FilterType::Nearest);

            let buffer: Vec<u32> = utils::gray_to_vec32(res_img);
            context
                .graphics_context
                .set_buffer(&buffer, win_width as u16, win_height as u16);
        }
    }
}

fn handle_input(
    context: &mut Context,
    win_height: u32,
    win_width: u32,
    state: ElementState,
    button: MouseButton,
) {
    if state == ElementState::Pressed && button == MouseButton::Left {
        if let ImageState::BeforeSelection(img) = &context.img_state {
            let square: u32 = utils::check_grid(
                context.mouse_pos.x,
                context.mouse_pos.y,
                win_width,
                win_height,
                context.args.n,
            );
            let square_width: f64 = img.width() as f64 / context.args.n as f64;
            let square_height: f64 = img.height() as f64 / context.args.n as f64;

            let x: u32 = ((square % context.args.n) as f64 * square_width).round() as u32;
            let y: u32 = ((square / context.args.n) as f64 * square_height).round() as u32;
            let cut_image =
                utils::cut_image(img, x, y, x + square_width as u32, y + square_height as u32);

            interpolation::bil_interpol(
                &context.args.intermediate,
                cut_image.as_raw(),
                cut_image.height(),
                cut_image.width(),
            );

            let interpolated_buffer = std::fs::read(&context.args.intermediate).unwrap();

            let interpolated_image: GrayImage = ImageBuffer::from_vec(
                cut_image.width() * 3 - 2,
                cut_image.height() * 3 - 2,
                interpolated_buffer,
            )
            .unwrap();

            if let Some(str) = &context.args.gray {
                interpolated_image.save(&str).unwrap();
            }

            context.img_state = ImageState::AfterSelection(interpolated_image);

            context.graphics_context.window().request_redraw();
        }
    }
}
