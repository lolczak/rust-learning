use std::f64::consts;

static MAX_HEALTH: i32 = 1_000;

static GAME_NAME: &'static str = "Monster Attack";

fn main() {
    const PI: f32 = 3.14;
    println!("The game: {0}, {1}", &GAME_NAME, PI);
    println!("Test pi: {:e}", consts::PI);
    println!("The max health is equal: {max:b}", max = MAX_HEALTH);
    foo();
}

fn foo() -> () {
    let _bar = 5u32;
    let pi = 3.14_f32;
    let million = 1_000_000;
    let empty = ();

    return ();
}