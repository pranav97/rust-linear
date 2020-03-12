use std::io;
use std::time;
extern crate num_cpus;
use std::sync::mpsc::channel;

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

fn take_stdin_matrix() -> Box<Matrix>  {
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
    return Box::new(m1);
}

fn multiply_row_and_col(a:std::sync::Arc<Box<Matrix>>, cur_row:usize, b:std::sync::Arc<Box<Matrix>> , cur_col:usize) -> i64 {
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


fn multiply_helper (a:std::sync::Arc<Box<Matrix>>, b:std::sync::Arc<Box<Matrix>>, start:usize, end:usize) -> Vec<(usize, i64)> {
    let mut rows:Vec<(usize, i64)> = vec![];
    for cur_row in start..end {
        for cur_col in 0..b.num_cols  {
            let arp_a = a.clone();
            let arp_b = b.clone();
            rows.push(
                (get_ind!(cur_row, cur_col, a.num_rows), multiply_row_and_col(arp_a, cur_row, arp_b, cur_col))
            );
        }
    }
    return rows;
}

fn row_split_multiply(a:Box<Matrix>, b:Box<Matrix>) -> Box<Matrix> {
    let last_row:usize = a.num_rows;
    let mut rows_per_core:usize;
    let mut start:usize = 0;
    let mut end:usize;
    // count logical cores this process could try to use
    let mut thread_count = num_cpus::get();
    if thread_count == 0 {
        thread_count = 1;
    }
    let total_nums = (a.num_rows * b.num_cols) + 1;
    let mut answer:Vec<i64> = vec![0; total_nums];
    let mut children = vec![];
    let mut rcvrs = vec![];


    rows_per_core =  last_row / thread_count;
    if last_row % thread_count != 0 {
        rows_per_core += 1;
    }
    end = start + rows_per_core;
    let arp_a = std::sync::Arc::new(a);
    let arp_b = std::sync::Arc::new(b);
    while end <= last_row {
        let arp_a = arp_a.clone();
        let arp_b = arp_b.clone();
        let (tx, rx) = channel();

        let child = std::thread::spawn(move || {
            let new_vec = multiply_helper(arp_a,
               arp_b,
               start,
               end
            );
            // println!("{:?}", new_vec);
            tx.send(new_vec.to_owned())
                .expect("Unable to send on channel");

        });

        children.push(child);
        rcvrs.push(rx);
        start += rows_per_core;
        end += rows_per_core
    }
    if end != last_row {
        let arp_a = arp_a.clone();
        let arp_b = arp_b.clone();
        let (tx, rx) = channel();
        let child = std::thread::spawn(move || {
            let a = multiply_helper(arp_a, arp_b, start, last_row);
            tx.send(a.to_owned())
                .expect("Unable to send on channel");
        });
        rcvrs.push(rx);
        children.push(child);
    }


    for r in rcvrs {
        let value = r.recv().expect("Unable to receive from channel");
        for v in value {
            let (ind, val) = v;
            answer[ind] = val;
        }
    }

    for child in children {
        child.join().unwrap();
    }
    let res = Matrix {
        num_rows: arp_a.num_rows,
        num_cols: arp_b.num_cols,
        mat: answer.into_boxed_slice()
    };
    return Box::new(res);

}


fn main() {
    let m1 = take_stdin_matrix();
    println!("You just entered: ");
    print_matrix(&m1);

    let m2 = take_stdin_matrix();
    println!("You just entered: ");
    print_matrix(&m2);

    if m1.num_cols != m2.num_rows {
        println!("Matrix Multiplication not possible");
        return;
    }
    let now = time::Instant::now();
    let m3 = row_split_multiply(m1, m2);
    println!("Time taken by multiplication: {} microseconds", now.elapsed().as_micros());

    // let m3_ptr:&Matrix = &m3;
    println!("Product is: ");
    print_matrix(&m3);
}

