use bracket_lib::prelude::*;

struct State {
    player: Player,
    frame_time: f32,
    game_mode: GameMode,
}

impl State {
    fn new() -> State {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            game_mode: GameMode::Menu,
        }
    }

    fn main_menu(&mut self, contex: &mut BTerm) {
        contex.cls();
        contex.print_centered(5, "Welcome to Flappy!");
        contex.print_centered(8, "(P) Play Game");
        contex.print_centered(9, "(Q) Quit Game");

        if let Some(key_press) = contex.key {
            match key_press {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => contex.quitting = true,
                _ => {}
            }
        }
    }
    fn play(&mut self, contex: &mut BTerm) {
        contex.cls_bg(NAVYBLUE); //Set color for window;
        self.frame_time += contex.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;

            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = contex.key {
            self.player.flap();
        }

        self.player.render(contex);
        contex.print(0, 0, "Press SPACE to flap.");

        if self.player.y > SCREEN_HEIGHT {
            self.game_mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.game_mode = GameMode::Playing;
    }

    fn lost(&mut self, contex: &mut BTerm) {
        contex.cls();
        contex.print_centered(5, "You lost!");
        contex.print_centered(8, "(P) Play Game");
        contex.print_centered(9, "(Q) Quit Game");

        if let Some(key_press) = contex.key {
            match key_press {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => contex.quitting = true,
                _ => {}
            }
        }
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Player {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, contex: &mut BTerm) {
        contex.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.5;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

//This is a trait from 'bracket_lib'
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.game_mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.lost(ctx),
        }
    }
}

fn main() -> BError {
    //we need to intialize the bracket_lib
    //describing the type of window and game loop we want to create.

    let context = BTermBuilder::simple80x50()
        .with_title("Flappy impl")
        .build()?;

    main_loop(context, State::new())
}
