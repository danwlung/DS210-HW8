//Question 2
//Collaborators: None

fn calculate_next_generation(board: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // Create a new board to store the next generation
    let mut new_board = vec![vec![false; 16]; 16];

    let n_rows = board.len();
    let n_cols = board[0].len();

    for i in 0..n_rows {
        for j in 0..n_cols {
            let live_neighbors = count_live_neighbors(board, i, j);

            if board[i][j] {
                if live_neighbors == 2 || live_neighbors == 3 {
                    new_board[i][j] = true;
                }
            } else {
                if live_neighbors == 3 {
                    new_board[i][j] = true;
                }
            }
        }
    }
    new_board
}

//Count the number of live neighbors to determine if the cell should be alive or dead in next generation
fn count_live_neighbors(board: &Vec<Vec<bool>>, x: usize, y: usize) -> u8 {
    let mut live_neighbors = 0;
    
    // Loop through all 8 neighbors
    let neighbor_x = [-1, -1, -1, 0, 0, 1, 1, 1];
    let neighbor_y = [-1, 0, 1, -1, 1, -1, 0, 1];

    let n_rows = board.len();
    let n_cols = board[0].len();

    // Loop through all 8 neighbors
    for k in 0..8 {
        // Calculate the coordinates of the neighbor
        let new_x = (x as isize + neighbor_x[k]) as usize;
        let new_y = (y as isize + neighbor_y[k]) as usize;

        if new_x < n_rows && new_y < n_cols {
            if board[new_x][new_y] {
                live_neighbors += 1;
            }
        }
    }

    live_neighbors
}

fn display_board(board: &Vec<Vec<bool>>) {
    for row in board {
        for &cell in row {
            if cell {
                print!("■ "); // Display an alive cell
            } else {
                print!("□ "); // Display a dead cell
            }
        }
        println!(); // Start a new line for the next row
    }
}

fn main() {
    let mut board = vec![vec![false; 16]; 16];

    // Set individual initial squares to "alive" (true)
    board[0][1] = true;
    board[1][2] = true;
    board[2][0] = true;
    board[2][1] = true;
    board[2][2] = true;

    for generation in 0..10 {
        println!("Generation {}:", generation+1);
        display_board(&board);
        println!();
        board = calculate_next_generation(&board);
    }
}