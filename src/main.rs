use piston_window::*;
use std::collections::HashMap;

struct Player {
    pos: (f64, f64),
    size: f64,
    velocity: (f64, f64),
    grounded: bool,
}

impl Player {
    fn from_pos(pos: (f64, f64), size: f64) -> Player {
        Player {
            pos,
            size,
            velocity: (0.0, 0.0),
            grounded: false,
        }
    }

    fn jump(&mut self) {
        if self.grounded {
            self.velocity.1 -= 10.0;
            self.grounded = false;
        }
    }

    fn handle_input(&mut self, input_state: &HashMap<Key, ButtonState>) {
        if let Some(ButtonState::Press) = input_state.get(&Key::Up) {
            self.jump();
        }
        if let Some(ButtonState::Press) = input_state.get(&Key::Right) {
            self.velocity.0 += 0.1;
        }
        if let Some(ButtonState::Press) = input_state.get(&Key::Left) {
            self.velocity.0 -= 0.1;
        }
    }

    fn update(&mut self, ground: &Ground, update_args: UpdateArgs) {
        let dt = update_args.dt * 10.0;

        let p1 = [self.pos.0, self.pos.1 + self.size];
        let p2 = [self.pos.0 + self.size, self.pos.1 + self.size];

        for &p in &[p1, p2] {
            //check if we're above or below a line
            let ([x1, y1], [x2, y2]) = (ground.vertices[0], ground.vertices[1]);

            let a = y2 - y1;
            let b = x2 - x1;
            let c = y1 * (x2 - x1);
            let distance = (a * p[0] - b * p[1] + c) / f64::sqrt(a.powi(2) + b.powi(2));
            println!("{}", distance);

            if distance < 0.0 && self.velocity.0 < 0.0 {
                // trying to go beneath
                self.velocity.0 = 0.0;
            }

            if distance < 1.0 && distance > -1.0 {
                self.grounded = true;
                self.velocity.1 = 0.0;
                break;
            } else {
                self.grounded = false;
            }
        }

        if !self.grounded {
            self.velocity.1 += 2.0 * dt;
        }

        self.pos.0 += self.velocity.0 * dt;
        self.pos.1 += self.velocity.1 * dt;
    }

    fn draw(&self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, _device| {
            let red = [1.0, 0.0, 0.0, 1.0];
            let blue = [0.0, 0.0, 1.0, 1.0];

            clear([1.0; 4], graphics);
            rectangle(
                blue,
                [self.pos.0, self.pos.1, self.size, self.size],
                context.transform,
                graphics,
            );

            ellipse(
                red,
                [self.pos.0 - 2.0, self.pos.1 + self.size - 2.0, 4.0, 4.0],
                context.transform,
                graphics,
            );
            ellipse(
                red,
                [
                    self.pos.0 + self.size - 2.0,
                    self.pos.1 + self.size - 2.0,
                    4.0,
                    4.0,
                ],
                context.transform,
                graphics,
            );
        });
    }
}

#[derive(Debug)]
struct Ground {
    vertices: Vec<[f64; 2]>,
    normals: Vec<[f64; 2]>,
}

impl Ground {
    fn draw(&self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, _device| {
            polygon(
                [0.2, 0.2, 0.2, 1.0],
                self.vertices.as_slice(),
                context.transform,
                graphics,
            );

            for (&point, &normal) in self.vertices.iter().zip(&self.normals) {
                line_from_to(
                    [1.0, 0.0, 0.0, 1.0],
                    1.0,
                    point,
                    math::add(point, math::mul_scalar(normal, 20.0)),
                    context.transform,
                    graphics,
                );
            }
        });
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut player = Player::from_pos((20.0, 0.0), 100.0);
    let ground = Ground {
        vertices: vec![[0.0, 240.0], [200.0, 300.0], [400.0, 400.0], [0.0, 999.9]],
        normals: vec![
            create_normal([0.0, 240.0], [200.0, 300.0]),
            create_normal([0.0, 240.0], [200.0, 300.0]),
        ],
    };
    println!("{:?}", ground.normals);

    let mut input_state: HashMap<Key, ButtonState> = HashMap::new();

    while let Some(event) = window.next() {
        if let Event::Input(Input::Button(button_args), _) = event {
            if let Button::Keyboard(key) = button_args.button {
                input_state.insert(key, button_args.state);
            }
        }

        player.handle_input(&input_state);

        if let Event::Loop(Loop::Update(update_args)) = event {
            player.update(&ground, update_args);
        }

        window.draw_2d(&event, |_context, graphics, _device| {
            clear([1.0; 4], graphics);
        });

        player.draw(&mut window, &event);
        ground.draw(&mut window, &event);
    }
}

fn create_normal(p1: [f64; 2], p2: [f64; 2]) -> [f64; 2] {
    let unnormalised = math::perp(math::sub(p1, p2));
    let length = f64::sqrt(unnormalised[0].powi(2) + unnormalised[1].powi(2));
    math::mul_scalar(unnormalised, 1.0 / length)
}

fn collision_point_line(point: [f64; 2], line: [[f64; 2]; 2]) -> bool {
    let epsilon = 0.05;
    distance(point, line[0]) + distance(point, line[1]) >= distance(line[0], line[1]) - epsilon
        && distance(point, line[0]) + distance(point, line[1])
            <= distance(line[0], line[1]) + epsilon
}

fn distance(p1: [f64; 2], p2: [f64; 2]) -> f64 {
    let dx = p2[0] - p1[0];
    let dy = p2[1] - p1[1];
    f64::sqrt(dx.powi(2) + dy.powi(2))
}
