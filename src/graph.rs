/// Takes data points and displays them on a graph.
/// An initial range for the values the Graph should expect has to be specified, but the the display will zoom to fit in any new points inside the graph
pub struct Graph {
    points: Vec<[f64; 2]>,
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    render_rect: [f64; 4],
    border_radius: f64, // square width & height
    point_radius: f64, // square width & height
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
            border_radius: 2.0,
            point_radius: 2.0,
        }
    }
    /// Graph will scale x and y to fit this point inside the displayed graph
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

            //draws the border
            let border_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            graphics::rectangle(border_color, [self.render_rect[0], self.render_rect[1], self.render_rect[2], self.border_radius], c.transform, gl);
            graphics::rectangle(border_color, [self.render_rect[0], self.render_rect[1], self.border_radius, self.render_rect[3]], c.transform, gl);
            graphics::rectangle(border_color, [self.render_rect[0], self.render_rect[1] + self.render_rect[3], self.render_rect[2], self.border_radius], c.transform, gl);
            graphics::rectangle(border_color, [self.render_rect[0] + self.render_rect[2], self.render_rect[1], self.border_radius, self.render_rect[3]], c.transform, gl);

            //graphs all the points
            let line_color: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
            for p in &self.points {
                let x: f64 = (p[0] - self.min_x)/(self.max_x - self.min_x) * (self.render_rect[2] - self.point_radius*2.0 - self.border_radius) + self.render_rect[0] + self.border_radius;
                let y: f64 = -(p[1] - self.min_y)/(self.max_y - self.min_y) * (self.render_rect[3] - self.point_radius*2.0 - self.border_radius) + self.render_rect[1] + self.render_rect[3] - 2.0*self.point_radius;
                let r = self.point_radius;
                graphics::rectangle(line_color, [x, y, r*2.0, r*2.0], c.transform, gl)
            }
        });
    }
}
