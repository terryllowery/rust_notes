#[allow(dead_code)]

/// Example taken from here: 
/// https://youtu.be/2sm7zdajzi8

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let player_direction:Direction = Direction::Up;

    // Match is like a switch statement
    match player_direction {
        Direction::Up => println!("We are heading up!"),
        Direction::Left => println!("We are headed left!"),
        Direction::Right => println!("We are headed right"),
        Direction::Down => println!("We are headed down!")
    }
}
