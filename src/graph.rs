pub struct Graph {
    points: Vec<[f64; 2]>,
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    render_rect: [f64; 4],
}
impl Graph {
    pub fn new(render_rect: [f64; 4], value_rect: [f64; 4]) -> Graph {
        Graph {
            points: Vec::new(),
            min_x: value_rect[0],
            max_x: value_rect[2] - value_rect[0],
            min_y: value_rect[1],
            max_y: value_rect[3] - value_rect[1],
            render_rect: render_rect,
        }
    }
    pub fn add_data_point(&mut self, point: [f64; 2]) {
        self.points.push(point);
        if point[0] < self.min_x {
            self.min_x = point[0];
        }
        if point[0] > self.max_x {
            self.max_x = point[0]
        }
        if point[1] < self.min_y {
            self.min_y = point[1];
        }
        if point[1] > self.max_y {
            self.max_y = point[1]
        }
    }

    pub fn draw(&self, r: &piston::RenderArgs, gl: &mut opengl_graphics::GlGraphics) {
        gl.draw(r.viewport(), |c, gl| {
            let line_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            for p in &self.points {
                let r: f64 = 10.0;
                let x: f64 = (p[0] - self.min_x)/(self.max_x - self.min_x) * self.render_rect[2] + self.render_rect[0];
                let y: f64 = -(p[1] - self.min_y)/(self.max_y - self.min_y) * self.render_rect[3] + self.render_rect[1] + self.render_rect[3];
                println!("x: {}, y: {}", x, y);
                graphics::rectangle(line_color, [x-r, y-r, r*2.0, r*2.0], c.transform, gl)
            }
        });
    }
}
