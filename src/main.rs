use bracket_lib::prelude::*;

struct State {}

//This is a trait from 'bracket_lib'
impl GameState for State {
    // ctx means -> context
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello from the main fn!!");
    }
}

fn main() -> BError {
    //we need to intialize the bracket_lib
    //describing the type of window and game loop we want to create.

    let context = BTermBuilder::simple80x50()
        .with_title("Flappy impl")
        .build()?;

    main_loop(context, State {})
}
