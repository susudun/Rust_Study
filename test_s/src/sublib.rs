pub use std::io;

#[derive(Debug)]
pub enum UserState{
Walk,
Jog,
Jump,
Down,
Falling,
Standing,
NoneState,
}

pub fn user_state_input() -> Result<String, io::Error> {
    println!("Enter user state (walk, jog, jump, down, falling, standing):");
    let mut input : String = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

