use piston_window::*;

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
            self.velocity.1 -= 1.0;
            self.grounded = false;
        }
    }

    fn update(&mut self, update_args: UpdateArgs) {
        if !self.grounded {
            self.velocity.1 += 1.0 * update_args.dt;
        }
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;
        if self.pos.1 > 140.0 {
            self.grounded = true;
            if self.velocity.1 > 0.0 {
                self.velocity = (0.0, 0.0);
            }
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

struct Ground;

impl Ground {
    fn draw(window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, _device| {
            rectangle(
                [0.2, 0.2, 0.2, 1.0],
                [0.0, 240.0, 640.0, 240.0],
                context.transform,
                graphics,
            );
        });
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut player = Player::from_pos((0.0, 0.0));

    while let Some(event) = window.next() {
        if let Event::Input(Input::Button(button_args), _) = event {
            if let ButtonArgs {
                button: Button::Keyboard(Key::Up),
                state: ButtonState::Press,
                scancode: _,
            } = button_args
            {
                println!("jumping");
                player.jump();
            }
        }

        if let Event::Loop(Loop::Update(update_args)) = event {
            player.update(update_args);
        }

        window.draw_2d(&event, |_context, graphics, _device| {
            clear([1.0; 4], graphics);
        });
        player.draw(&mut window, &event);
        Ground::draw(&mut window, &event);
    }
}
