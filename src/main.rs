fn mul_matrix(matrix0: [u64; 4], matrix1: [u64; 4], result: &mut [u64; 4]) {
    result[0] = matrix0[0] * matrix1[0] + matrix0[1] * matrix1[2];
    result[1] = matrix0[0] * matrix1[1] + matrix0[1] * matrix1[3];
    result[2] = matrix0[2] * matrix1[0] + matrix0[3] * matrix1[2];
    result[3] = matrix0[2] * matrix1[1] + matrix0[3] * matrix1[3];
}

fn pow_matrix(matrix: [u64; 4], n: u64, result: &mut [u64; 4]) {
    if n == 0 {
        result[0] = 1;
        result[1] = 0;
        result[2] = 0;
        result[3] = 1;
    } else if n == 1 {
        result[0] = matrix[0];
        result[1] = matrix[1];
        result[2] = matrix[2];
        result[3] = matrix[3];
    } else if n % 2 == 0 {
        let mut tmp: [u64; 4] = [0, 0, 0, 0];
        mul_matrix(matrix, matrix, &mut tmp);
        pow_matrix(tmp, n / 2, result);
    } else {
        let mut tmp: [u64; 4] = [0, 0, 0, 0];
        let mut tmp2: [u64; 4] = [0, 0, 0, 0];
        mul_matrix(matrix, matrix, &mut tmp);
        pow_matrix(tmp, n / 2, &mut tmp2);
        mul_matrix(matrix, tmp2, result);
    }
}

fn calc_fib(n: u64) -> u64 {
    let matrix: [u64; 4] = [1, 1, 1, 0];
    let mut result: [u64; 4] = [0, 0, 0, 0];
    pow_matrix(matrix, n, &mut result);
    return result[2] + result[3];
}
fn main() {
    loop {
        println!("何番目のフィボナッチ数列を求めますか(F(n=0)=1,F(n=1)=1)＞");
        let mut fib_num = String::new();
        std::io::stdin()
            .read_line(&mut fib_num)
            .expect("Failed to Read Line");

        let fib_num: u64 = match fib_num.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => continue,
        };
        if fib_num < 1{
            println!("Error: Input num > 0");
            continue;
        }

        println!(
            "{}番目のフィボナッチ数列の値は{}",
            fib_num,
            calc_fib(fib_num - 1)
        );
        break;
    }
}
