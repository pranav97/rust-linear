use std::io;
use std::time;


macro_rules! get_ind {
    ($cur_row: expr, $cur_col: expr, $num_rows: expr) => (($num_rows * $cur_col) + $cur_row);
}


struct Matrix {
    num_rows: usize,
    num_cols: usize,
    mat: Box<[i64]>
}

trait Show {
    fn show(&self) -> String;
}

fn print_matrix(m:&Matrix) {
    for row in 0..m.num_rows{
        for col in 0..m.num_cols {
            print!("{} \t", m.mat[get_ind!(row, col, m.num_rows)]);
        }
        println!("");
    }
}


fn input_num()  -> i64 {
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i64 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    return guess;
}

fn input_len()  -> usize{
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: usize = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    return guess;
}


fn take_mat_input(num_rows: usize, num_cols: usize) -> Box<[i64]> { 
    println!("Enter {} numbers ", num_rows * num_cols);
    let total_nums: usize = num_rows * num_cols;
    let mut mat:Box<[i64]> = vec![0; total_nums].into_boxed_slice();
    let mut cur_row: usize;
    let mut cur_col: usize;
    cur_row = 0;
    while cur_row < num_rows {
        cur_col = 0;
        while cur_col < num_cols {
            let cur_num: i64 = input_num();
            let temp:usize = get_ind!(cur_row, cur_col, num_rows);
            mat[temp] = cur_num;
            cur_col+=1;
        }
        cur_row+=1;
    }
    return mat;
}

fn take_stdin_matrix() -> Matrix  {
    println!("Enter the number of rows (m)");
    let rows: usize = input_len();
    println!("Enter the number of cols (n)");
    let cols: usize = input_len();
    let vec1 = take_mat_input(rows, cols);
    let m1 = Matrix {
        num_rows: rows,
        num_cols: cols,
        mat: vec1,
    };
    return m1;
}

fn multiply_row_and_col(a:&Matrix, cur_row:usize, b:&Matrix, cur_col:usize) -> i64 {
    let mut val:i64 = 0;
    let mut cc:usize = get_ind!(0, cur_col, b.num_rows);
    let last_ind:usize = get_ind!(cur_row, a.num_cols-1, a.num_rows); 
    let mut cur_ind:usize = get_ind!(cur_row, 0, a.num_rows);
    while cur_ind <= last_ind {
        val += a.mat[cur_ind] * b.mat[cc];
        cc += 1;
        cur_ind += a.num_rows
    }
    return val;
}

fn multiply_single_thread(m1:&Matrix, m2:&Matrix)  -> Matrix {
    let sz = m1.num_rows * m2.num_cols;
    let mut res = Matrix {
        num_rows: m1.num_rows,
        num_cols: m2.num_cols,
        mat: vec![0; sz].into_boxed_slice()

    };
    let m1_ptr: &Matrix = &m1;
    let m2_ptr: &Matrix = &m2;
    for cur_row in 0..res.num_rows {
        for cur_col in 0..res.num_cols {
            let v:i64 = multiply_row_and_col(m1_ptr, cur_row, m2_ptr, cur_col);
            res.mat[get_ind!(cur_row, cur_col, res.num_rows)] = v;
        }
    }
    return res;
}

fn main() {
    let m1 = take_stdin_matrix();
    let m1_ptr:&Matrix = &m1;
    println!("You just entered: ");
    print_matrix(m1_ptr);

    let m2 = take_stdin_matrix();
    let m2_ptr:&Matrix = &m2;
    println!("You just entered: ");
    print_matrix(m2_ptr);

    if m1.num_cols != m2.num_rows {
        println!("Matrix Multiplication not possible");
        return;
    }
    let now = time::Instant::now();
    let m3 = multiply_single_thread(m1_ptr, m2_ptr);
    println!("Time taken by multiplication: {} microseconds", now.elapsed().as_micros());

    let m3_ptr:&Matrix = &m3;
    println!("Product is: ");
    print_matrix(m3_ptr);
}

