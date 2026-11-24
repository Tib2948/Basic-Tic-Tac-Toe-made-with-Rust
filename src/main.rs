use std::io::stdin;

enum Turn {
    X,
    O,
}

impl Turn {
    fn flipped(&self) -> Self {
        match *self {
            Turn::X => Turn::O,
            Turn::O => Turn::X,
        }
    }

    fn to_char(&self) -> char {
        match &self {
            Turn::X => 'X',
            Turn::O => 'O',
        }
    }
}

fn format_and_display_gamebox(state: [char; 9])
// returns a formatted string of the gamebox to be printed / "displayed"
{
    let [a1, a2, a3, b1, b2, b3, c1, c2, c3] = state;

    let gamebox_string = format!(
        "+---+---+---+\n| {a1} | {a2} | {a3} |\n+---+---+---+\n| {b1} | {b2} | {b3} |\n+---+---+---+\n| {c1} | {c2} | {c3} |\n+---+---+---+"
    );

    println!("{}", gamebox_string);
}

fn get_input() -> usize // modifies the value of input
{
    let mut buffer = String::new();

    loop {
        stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input");
        match buffer.trim().parse::<usize>() {
            Ok(result) => {
                if (1..=3).contains(&(result % 10)) && (1..=3).contains(&(result / 10)) {
                    break result;
                } else {
                    println!("Input is outside of the bounds of the game. Try again");
                    buffer.clear();
                    continue;
                }
            }
            Err(_) => {
                println!("Error trying to get input, please try again:");
                buffer.clear();
                continue;
            }
        }
    }
}

fn modify_state(input: usize, gamebox_state: &mut [char; 9], turn: &Turn) -> bool {
    let column = input % 10 - 1;
    let row = input / 10 - 1;
    match gamebox_state[row * 3 + column] {
        ' ' => {
            gamebox_state[row * 3 + column] = turn.to_char();
            true
        }
        _ => {
            println!("Wrong input");
            false
        }
    }
}

fn check_win_condition(gamebox_state: &[char; 9]) -> bool {
    let mut won = false;

    let mut i: usize = 0;
    while i < 3 && won == false {
        if gamebox_state[i] == gamebox_state[i + 3]
            && gamebox_state[i] == gamebox_state[i + 6]
            && gamebox_state[i] != ' '
        {
            won = true;
            break;
        }

        i += 1;
    }

    i = 0;
    while i <= 6 && won == false {
        if gamebox_state[i] == gamebox_state[i + 1]
            && gamebox_state[i] == gamebox_state[i + 2]
            && gamebox_state[i] != ' '
        {
            won = true;
            break;
        }

        i += 3;
    }

    if won == false
        && gamebox_state[0] == gamebox_state[4]
        && gamebox_state[0] == gamebox_state[8]
        && gamebox_state[0] != ' '
    {
        won = true;
    }

    if won == false
        && gamebox_state[2] == gamebox_state[4]
        && gamebox_state[2] == gamebox_state[6]
        && gamebox_state[2] != ' '
    {
        won = true;
    }

    won
}

fn check_draw(gamebox_state: &[char; 9]) -> bool {
    let mut draw = true;

    for cell in gamebox_state {
        if *cell == ' ' {
            draw = false;
            break;
        }
    }

    draw
}

fn main() {
    let mut gamebox_state = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']; // initialize game state
    let mut game_turn = Turn::X;

    loop {
        format_and_display_gamebox(gamebox_state);

        if modify_state(get_input(), &mut gamebox_state, &game_turn) {
            game_turn = game_turn.flipped();
        } else {
            println!("Turn is being repeated");
            continue;
        };

        if check_win_condition(&gamebox_state) {
            format_and_display_gamebox(gamebox_state);
            println!("{} has won!", game_turn.flipped().to_char());
            break;
        }

        if check_draw(&gamebox_state) {
            println! {"It s a draw! Game over!"};
            format_and_display_gamebox(gamebox_state);
            break;
        }
    }

    println!("Press Enter to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());
}
