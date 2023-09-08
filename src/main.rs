use std::io::{self, Write};

#[derive(Copy, Clone, Debug, PartialEq)]
enum TableElement {
    X,
    O,
    None
}

#[derive(Debug, Clone)]
enum Player {
    X,
    O
}

enum PerformResult {
    Res(GameResult),
    InputOutOfBounds,
    PlaceTaken
}

#[derive(Clone, Debug)]
enum GameResult {
    Player(Player),
    Tie,
    None
}

fn main() {
    let mut table: [TableElement; 9] = [TableElement::None; 9];
    let mut turn: Player = Player::X;
    let mut msg: String = "".into();
    let mut winner: GameResult = GameResult::None;

    loop {
        clear_screen();
        println!("{}\n", msg);

        print_table(table);

        match winner {
            GameResult::Player(p) => {
                println!("\nPlayer {} has won!", player_to_string(&p));
                return;
            }
            GameResult::Tie => {
                println!("\nTie!");
                return;
            }
            GameResult::None => {
                println!("\nTurn: {:?}", turn);
            },
        }

        let mut player_input: String = String::new();
        input("> ", &mut player_input);

        let pos = match player_input.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                msg = format!("Invalid input: '{}'. The input is not a number.", player_input).into();
                continue;
            }
        };

        match perform(&mut table, &turn, pos) {
            PerformResult::Res(res) => {
                winner = res.clone();

                if let GameResult::None = res {
                    turn = swap_players(&turn);
                }
            }

            PerformResult::InputOutOfBounds => {
                msg = format!("Invalid input: '{}'. Position is out of bounds.", player_input).into();
            }

            PerformResult::PlaceTaken => {
                msg = format!("Invalid input: '{}'. That place has already been taken.", player_input).into();
            }
        }
    }
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn print_table(table: [TableElement; 9]) {
    for i in 0..3 {
        let ii: usize = i * 3;
        println!(" {} | {} | {}", element_to_string(table[ii], ii + 1), element_to_string(table[ii + 1], ii + 2), element_to_string(table[ii + 2], ii + 3));

        if i != 2 {
            println!("---+---+---");
        }
    }
}

fn perform(table: &mut [TableElement; 9], player: &Player, pos: usize) -> PerformResult {
    if pos < 1 || pos > 9 {
        return PerformResult::InputOutOfBounds;
    }
    else if table[pos - 1] != TableElement::None {
        return PerformResult::PlaceTaken;
    }

    table[pos - 1] = player_to_element(player);
    println!("{:?}", table);
    PerformResult::Res(check_win(table))
}

fn check_win(table: &[TableElement; 9]) -> GameResult {
    if table[0] == table[1] && table[1] == table[2] && table[0] != TableElement::None {
        GameResult::Player(element_to_player(&table[0]))
    }
    else if table[3] == table[4] && table[4] == table[5] && table[3] != TableElement::None {
        GameResult::Player(element_to_player(&table[3]))
    }
    else if table[6] == table[7] && table[7] == table[8] && table[6] != TableElement::None {
        GameResult::Player(element_to_player(&table[6]))
    }

    else if table[0] == table[3] && table[3] == table[6] && table[0] != TableElement::None {
        GameResult::Player(element_to_player(&table[0]))
    }
    else if table[1] == table[4] && table[4] == table[7] && table[1] != TableElement::None {
        GameResult::Player(element_to_player(&table[1]))
    }
    else if table[2] == table[5] && table[5] == table[8] && table[2] != TableElement::None {
        GameResult::Player(element_to_player(&table[2]))
    }

    else if table[0] == table[4] && table[4] == table[8] && table[0] != TableElement::None {
        GameResult::Player(element_to_player(&table[0]))
    }
    else if table[2] == table[4] && table[4] == table[6] && table[2] != TableElement::None {
        GameResult::Player(element_to_player(&table[2]))
    }

    else {
        for el in table {
            if *el == TableElement::None {
                return GameResult::None;
            }
        }

        GameResult::Tie
    }
}

fn swap_players(player: &Player) -> Player {
    match player {
        Player::X => Player::O,
        Player::O => Player::X
    }
}

fn player_to_element(player: &Player) -> TableElement {
    match player {
        Player::X => TableElement::X,
        Player::O => TableElement::O
    }
}

fn player_to_string(player: &Player) -> String {
    match player {
        Player::X => "X".into(),
        Player::O => "O".into()
    }
}

fn element_to_player(element: &TableElement) -> Player {
    match element {
        TableElement::X => Player::X,
        TableElement::O => Player::O,

        _ => panic!("aqui vc n chega")
    }
}

fn element_to_string(element: TableElement, count: usize) -> String {
    match element {
        TableElement::X => "X".into(),
        TableElement::O => "O".into(),
        TableElement::None => count.to_string()
    }
}

fn input(prompt: &str, output: &mut String) {
    print!("{}", prompt);

    io::stdout().flush().unwrap();
    io::stdin().read_line(output).unwrap();
    *output = output.trim().into();
}
