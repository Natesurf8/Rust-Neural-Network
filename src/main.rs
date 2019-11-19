extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod graph;
use graph::Graph;

mod network;

fn main() {
    let gl_version = opengl_graphics::OpenGL::V3_2;
    let mut window: glutin_window::GlutinWindow = piston::window::WindowSettings::new(
            "neural_net",
            [800, 600]
        )
        .graphics_api(gl_version)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = opengl_graphics::GlGraphics::new(gl_version);

    // this will measure the performance of the network in real time
    let mut performance_graph = Graph::new([200.0, 200.0, 500.0, 300.0], [3.0, 3.0, 1.0, 1.0]);

    // try graphing some dummy data
    for x in 0..=20 {
        performance_graph.add_data_point([x as f64, ((x-10)*(x-10)-5) as f64])
    }

    //todo: is this traitwise methodology necessary
    let mut events = piston::Events::new(piston::EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        use piston::RenderEvent;
        if let Some(r) = e.render_args() {
            //draw(&r, &mut gl);
            performance_graph.draw(&r, &mut gl);
        }
    }
    println!("Hello, world!");
}
