use std::io;

fn sieve_of_eratosthenes(mut primes: Vec<bool>, n: usize) {
    for i in 2..=n {
        if primes[i] {
            print!("{} ", i); // i가 소수임을 출력
            for j in (i*i..=n).step_by(i) {
                primes[j] = false; // i의 배수를 모두 지움
            }
        }
    }
}

fn main() {
    let mut n = String::new(); // 빈 문자열 생성
    println!("Find prime numbers up to: ");
    io::stdin()
    .read_line(&mut n) // 입력받고 n에 저장
    .expect("Failed to read input"); // 에러 날 시 띄워지는 메시지
    let n: usize = n.trim()
    .parse()
    .expect("Invalid input!!");

    let primes = vec![true; n+1]; // 모든 수를 소수로 가정
    println!("Prime numbers up to {}:", n);
    sieve_of_eratosthenes(primes, n);
    println!("\n");
}
