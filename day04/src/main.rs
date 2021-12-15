use anyhow::Result;

mod helpers;
use helpers::parse_file;

type Board = Vec<Vec<i32>>;

pub fn winning_board(numbers_drawn: Vec<i32>, boards: Vec<Board>) -> Result<Vec<Vec<i32>>> {

    for mut board in boards {
        for n in numbers_drawn.iter() {
            for row in 0..board.len() {
                let mut row_cnt = 0;
                let mut col_cnt = 0;
                for col in 0..board[0].len() {
                    if board[row][col] == *n { // board[0]: original (elements in the same column)
                        col_cnt += 1;
                        board[row][col] = -1;
                    }
                    if board[col][row] == *n { // board[1]: transposed (elements in the same row)
                        row_cnt += 1;
                        board[col][row] = -1;
                    }
                }
                if (row_cnt == board.len()) || (col_cnt == board.len()) {
                    return Ok(board);
                }
            }
        }
    }
    
    panic!("No winning board");
}

fn main() -> Result<()> {

    let lines = parse_file("input")?;
    for line in lines {
        println!("{}", line);
    }

    // Board: Vec<Vec<i32>>
    // for line in lines {
    //     println!("{}", line);
    // }

    /* Algorithm:
        1. only rows and columns are considered 
 
        2. 
        
        3. n * board.into_iter()
            .map_filter(|e| e != -1)
            .sum()
    */
    
    // let board = winning_board(numbers_drawn, boards)?;

    Ok(())
}
