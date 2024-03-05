use std::io;

fn game(mut win: bool, mut round: u8) {
    let mut board: Vec<Vec<char>> = vec![vec![' '; 3]; 3];
    let mut first_player: char = ' ';
    let mut second_player: char = ' ';
    let mut point: (usize, usize) = (0, 0);

    println!("Hello");
    input(&mut first_player, &mut second_player);

    while !win && round < 9 {
        println!("Enter position eg.A1: ");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Error");
        point.0 = user_input.chars().nth(0).unwrap() as usize - 'A' as usize;
        point.1 = user_input.chars().nth(1).unwrap() as usize - '1' as usize;
        if validate(point) && board[point.0][point.1] == ' ' {
            update(&mut board, point, if round % 2 == 0 { first_player } else { second_player });
            round += 1;
            win = check(&board);
            print_board(&board);
        }
        else { eprintln!("Unexpected values!") }
    }
}

fn input(first_player: &mut char, second_player: &mut char) {
    while *first_player != 'x' && *first_player != 'o' {
        println!("Do you prefer play as a circle[o] or a cross[x]?");
        let mut option = String::new();
        let _ = io::stdin().read_line(&mut option);
        *first_player = option.chars().nth(0).unwrap();
    };
    *second_player = if *first_player == 'x' { 'o' } else { 'x' };
}

fn validate( point: (usize, usize)) -> bool{
    if point.0 > 2 || point.1 > 2{
        return false;
    }
    return true
}

fn update(board: &mut Vec<Vec<char>>, point: (usize, usize), sign: char){
    board[point.0][point.1] = sign;
    println!("B: {} ",board[point.0][point.1])
}
fn check(board: &Vec<Vec<char>> ) -> bool{
    for i in 0..3{
        if board[i][0] != ' ' && board[i][0] == board[i][1] && board[i][1]==board[i][2]{
            return true
        }
        if board[0][i] != ' ' && board[0][i] == board[1][i] && board[1][i]==board[2][i]{
            return true
        }
    }
    if board[0][0] != ' ' && board[0][0] == board[1][1] && board[1][1]==board[2][2]{
        return true
    }
    if board[0][2] != ' ' && board[0][2] == board[1][1] && board[1][1]==board[2][0]{
        return true
    }
    return false
}

fn print_board(board: &Vec<Vec<char>>) {
    for row in board{
        for cell in row{
            print!("|{}",cell);
        }
        println!("|");
    }
    println!()
}

fn result(win: bool, round: u8) {
    if !win {
        println!("DRAW");
    } else {
        println!("Win {}", if round % 2 == 1 { "First player" } else { "Second player" });
    }
}

fn main() {
    let win = false;
    let  round: u8 = 0;
    game(win, round);
    result(win, round);
}





