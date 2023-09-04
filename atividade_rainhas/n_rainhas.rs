//pseudocódigo:
//
//is_safe (tabuleiro, linha, coluna) {
//  check_same_col
//  check_diagonal_upper_left
//  check_diagonal_upper_right
//}
//
//solve_n_queens_util (board, row){
//  if row => board.len()
//    return true;
//
//  foreach col{
//    if is_safe{
//      board[row][col] = true
//
//      if (solve_n_queens_util)
//        return true;
//
//      board[row][col] = false
//    }
//  }
//  false
//}
//
//


//declaração de is_safe
fn is_safe(board: &Vec<Vec<bool>>, row: usize, col: usize) -> bool {

        //verifica se não há nenhuma rainha atacando na mesma coluna
        for i in 0..row {
                if board[i][col] {
                        return false;
                }
        }

        //verifica a diagonal superior esquerda
        //'as isize' and 'as usize' foi utilizado porque há uma checagem de >= 0. E não faz sentido checar
        //assim algo que não pode ser nunca menor que 0
        let mut i = row as isize - 1;
        let mut j = col as isize - 1;
        while i >= 0 && j >= 0 {
                if board[i as usize][j as usize] {
                        return false;
                }
                i -= 1;
                j -= 1;
        }

        //verifica a diagonal superior direita
        i = row as isize - 1;
        j = col as isize + 1;
        while i >= 0 && j < board.len() as isize {
                if board[i as usize][j as usize] {
                        return false;
                }
                i -= 1;
                j += 1;
        }

        //não foi encontrado "conflito"
        true
}

fn solve_n_queens_util(board: &mut Vec<Vec<bool>>, row: usize) -> bool {
        //n = tamanho do tabuleiro
        let n = board.len();

        //se a checagem passou da borda do tabuleiro
        if row >= n {

                //todas as rainhas estão colocadas ao passar por aqui
                return true; 
        }

        //precisamos checar todas as colunas dessa linha para saber em qual devemos colocar a
        //rainha
        for col in 0..n {
                if is_safe(board, row, col) {
                        
                        //se encontrarmos um lugar "seguro" então a rainha é colocada nele
                        board[row][col] = true;

                        //chamamos a próxima execução para colocar a rainha na próxima linha
                        //e esperamos que true seja retornado para retornarnos true nessa execução
                        if solve_n_queens_util(board, row + 1) {
                                return true;
                        }

                        //caso true não seja retornado, tiramos a rainha do tabuleiro e
                        //retornamos falso depois. Aqui está o backtrack
                        board[row][col] = false;
                }
        }

        //retornando falso
        false
}

//função auxiliar
fn solve_n_queens(n: usize) -> Option<Vec<Vec<bool>>> {
        let mut board = vec![vec![false; n]; n];

        if solve_n_queens_util(&mut board, 0) {
                Some(board)
        } else {
                None
        }
}

//função de teste
fn main() {
        //tamanho do tabuleiro e número de rainhas
        let n = 8;
        if let Some(solution) = solve_n_queens(n) {
                for row in solution {
                        for cell in row {
                                print!("{} ", if cell { "Q" } else { "." });
                        }
                        println!();
                }
        } else {
                println!("Não há solução para {}-rainhas.", n);
        }
}

