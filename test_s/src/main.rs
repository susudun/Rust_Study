mod sublib;
use sublib::*;



struct Users {
    name: String,
    state: UserState,
}

fn user_state_setting(state : Result<String,io::Error>) -> Option<UserState>{
    use sublib::UserState::*;
    match state {
    Ok(data_input) => match data_input.as_str() {
        "walk" => Some(Walk),
        "jog" => Some(Jog),
        "jump" => Some(Jump),
        "down" => Some(Down),
        "falling" => Some(Falling),
        "standing" => Some(Standing),
        _ => None,
    },
    Err(e) => {
        println!("Error reading input: {}", e);
        None
    }
}

}


fn main() {
    let mut user = Users {
        name: String::from("Alice"),
        state: UserState::NoneState,
    }; 
    let mut u_state = user_state_setting(sublib::user_state_input());
     match u_state {
        Some(state) => {
            user.state = state;
            println!("User {} is now in state: {:?}", user.name, user.state);
        },
        None => {
            println!("Invalid user state input.");
        }
    }
}