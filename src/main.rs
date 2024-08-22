use bracket_lib::prelude::*;

struct State {
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    game_mode: GameMode,
    score: i32,
}

impl State {
    fn new() -> State {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            game_mode: GameMode::Menu,
            score: 0,
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

        self.obstacle.render(contex, self.player.x);
        self.player.render(contex);

        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        contex.print(0, 0, "Press SPACE to flap.");
        contex.print(0, 1, &format!("Score: {}", self.score));

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.game_mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.frame_time = 0.0;
        self.game_mode = GameMode::Playing;
    }

    fn lost(&mut self, contex: &mut BTerm) {
        contex.cls();
        contex.print_centered(5, "You lost!");
        contex.print_centered(6, &format!("You got {} points", self.score));
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

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Obstacle {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40), //Defines epicenter to expand gap
            size: i32::max(2, 20 - score), //By doing this the gap will shrink the higher the
                                         //player score is but never less than 2;
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;

        return does_x_match && (player_above_gap || player_below_gap);
    }

    fn render(&mut self, context: &mut BTerm, player_x: i32) {
        // By subtracting the player x from
        let screen_x = self.x - player_x; //translating from world-space to screen-space;
        let half_size = self.size / 2;

        //Gap being the center we need to render top from 0 to gap - size/2 and bottom from gap +
        //size/2;
        for y in 0..self.gap_y - half_size {
            context.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            context.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }
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
            self.velocity += 0.2;
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
