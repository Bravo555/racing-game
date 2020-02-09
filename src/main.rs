use piston_window::*;
use std::collections::HashMap;

struct Player {
    pos: (f64, f64),
    velocity: (f64, f64),
    grounded: bool,
}

impl Player {
    fn from_pos(pos: (f64, f64)) -> Player {
        Player {
            pos,
            velocity: (0.0, 0.0),
            grounded: false,
        }
    }

    fn jump(&mut self) {
        if self.grounded {
            self.velocity.1 -= 50.0;
            self.grounded = false;
        }
    }

    fn update(&mut self, update_args: UpdateArgs, input_state: &HashMap<Key, ButtonState>) {
        let dt = update_args.dt * 10.0;

        if let Some(ButtonState::Press) = input_state.get(&Key::Up) {
            self.jump();
        }
        
        if let Some(ButtonState::Press) = input_state.get(&Key::Right) {
            self.velocity.0 += 2.0 * dt;
        }
        
        if let Some(ButtonState::Press) = input_state.get(&Key::Left) {
            self.velocity.0 -= 2.0 * dt;
        }

        if !self.grounded {
            self.velocity.1 += 9.81 * dt;
        }
        self.pos.0 += self.velocity.0 * dt;
        self.pos.1 += self.velocity.1 * dt;
        if self.pos.1 > 140.0 {
            self.grounded = true;
            if self.velocity.1 > 0.0 {
                self.velocity.1 = 0.0;
            }
        } else {
            self.grounded = false;
        }
    }

    fn draw(&self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [self.pos.0, self.pos.1, 100.0, 100.0],
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
                line_from_to([1.0, 0.0, 0.0, 1.0], 1.0, point, math::add(point, math::mul_scalar(normal, 20.0)), context.transform, graphics);
            }
        });
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut player = Player::from_pos((0.0, 0.0));
    let ground = Ground {
        vertices: vec![
            [0.0, 240.0],
            [200.0, 300.0],
            [400.0, 400.0],
            [0.0, 999.9],
        ],
        normals: vec![
            create_normal([0.0, 240.0], [200.0, 300.0]),
            create_normal([0.0, 240.0], [200.0, 300.0]),
        ]
    };
    println!("{:?}", ground.normals);

    let mut input_state: HashMap<Key, ButtonState> = HashMap::new();

    while let Some(event) = window.next() {
        if let Event::Input(Input::Button(button_args), _) = event {
            if let Button::Keyboard(key) = button_args.button {
                input_state.insert(key, button_args.state);
            }
        }

        if let Event::Loop(Loop::Update(update_args)) = event {
            player.update(update_args, &input_state);
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
    math::mul_scalar(unnormalised, 1.0/length)
}