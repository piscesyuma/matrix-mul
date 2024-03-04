use num_cpus;
use rand::Rng;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::SystemTime;

fn main() {
    let mat1 = get_random_matrix(500, 500);

    let mat2 = get_random_matrix(500, 500);

    let m = mat1.len();
    let n = mat1[0].len();

    // Time start
    let now = SystemTime::now();

    let _mat3 = mat_mul(&mat1, &mat2);

    match now.elapsed() {
        Ok(elapsed) => {
            println!(
                "Single thread calculation executed in {}ms",
                elapsed.as_millis()
            );
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    let now = SystemTime::now();
    let _mat4 = mat_mul_pal(&mat1, &mat2);

    // Time end
    match now.elapsed() {
        Ok(elapsed) => {
            println!(
                "Multi thread calculation executed in {}ms",
                elapsed.as_millis()
            );
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    // print!("{:?}\n", mat3);
    // print!("======================\n");
    // print!("{:?}\n", mat4);
}

// Single Threaded
fn mat_mul(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    #[allow(unused_mut)]
    let mut ans: Vec<Vec<i32>> = vec![];
    if m1.is_empty() || m2.is_empty() {
        return ans;
    }
    let m = m1[0].len();
    let n = m2.len();
    if n != m {
        return ans;
    }
    let mut ans: Vec<Vec<i32>> = vec![];
    for i in 0..n {
        ans.push(vec![]);
        for j in 0..n {
            let mut cur = 0;
            for k in 0..n {
                cur += m1[i][k] * m2[k][j];
            }
            ans[i].push(cur);
        }
    }
    ans
}

// Multi Threaded
fn mat_mul_pal(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    #[allow(unused_mut)]
    if m1.is_empty() || m2.is_empty() {
        return vec![];
    }
    let m = m1[0].len();
    let n = m2.len();

    if n != m {
        return vec![];
    }

    // Threading stuff
    let thread_count = num_cpus::get();
    let mut threads: Vec<_> = Vec::new();
    let (tx, rx) = mpsc::channel();
    let amat1 = Arc::new(m1.clone());
    let amat2 = Arc::new(m2.clone());

    for th in 0..thread_count {
        let tx = tx.clone();
        let amat1 = Arc::clone(&amat1);
        let amat2 = Arc::clone(&amat2);
        threads.push(thread::spawn(move || {
            println!("thread-{} started!", th);

            let start = (th * n) / thread_count;
            let end = (n * (th + 1)) / thread_count;
            let mut ans: Vec<Vec<i32>> = vec![vec![]; n];
            for i in start..end {
                for j in 0..n {
                    let mut cur = 0;
                    for k in 0..n {
                        cur += amat1[i][k] * amat2[k][j];
                    }
                    ans[i].push(cur);
                }
            }
            tx.send((start, end, ans)).unwrap();
        }));
    }

    let mut ans: Vec<Vec<i32>> = vec![vec![]; n];
    for v in rx.iter().take(thread_count) {
        let (start, end, mat) = v;
        for i in start..end {
            ans[i].extend(&mat[i]);
        }
    }

    for handle in threads {
        handle.join().unwrap();
    }

    ans
}

fn get_random_matrix(m: u32, n: u32) -> Vec<Vec<i32>> {
    let mut vec: Vec<Vec<i32>> = Vec::with_capacity(m as usize);

    for _ in 0..m {
        let mut vec_sub: Vec<i32> = Vec::with_capacity(n as usize);
        for _ in 0..n {
            vec_sub.push(rand::thread_rng().gen_range(0..1000))
        }
        vec.push(vec_sub);
    }

    return vec;
}
