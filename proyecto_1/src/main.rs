use softbuffer::GraphicsContext;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

mod interpolation;

fn main() {
    let b = 6;
    let mut buf: [u8; 3] = [1, 2, 3];
    let sum = interpolation::bil_interpol("result.img", &mut buf, 1, 3);

    println!("9 = {}", sum);
    println!("6 = {}", b);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(window) }.unwrap();

    let mut m_pos = PhysicalPosition::<f64>::new(0.0, 0.0);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::RedrawRequested(window_id) if window_id == graphics_context.window().id() => {
                let (width, height) = {
                    let size = graphics_context.window().inner_size();
                    (size.width, size.height)
                };
                let buffer: Vec<u32> = (0..((width * height) as usize))
                    .map(|index| {
                        let y = index / (width as usize);
                        let x = index % (width as usize);
                        let red = x % 255;
                        let green = y % 255;
                        let blue = (x * y) % 255;

                        let color = blue | (green << 8) | (red << 16);

                        color as u32
                    })
                    .collect::<Vec<u32>>();

                graphics_context.set_buffer(&buffer, width as u16, height as u16);
            }

            Event::WindowEvent { event, window_id }
                if window_id == graphics_context.window().id() =>
            {
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::CursorMoved { position, .. } => {
                        m_pos = position;
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        if state == ElementState::Pressed && button == MouseButton::Left {
                            println!("{:?}", m_pos);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}
