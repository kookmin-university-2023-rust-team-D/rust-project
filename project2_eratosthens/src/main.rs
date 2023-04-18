use std::io;
fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("error");
    let n: usize = n.trim().parse().unwrap();

    let mut primes = vec![true; n+1]; // 모든 수를 소수로 가정

    primes[0] = false; // 0은 소수가 아님
    primes[1] = false; // 1은 소수가 아님

    for i in 2..=n {
        if primes[i] {
            println!("{}", i); // i가 소수임을 출력
            for j in (i*i..=n).step_by(i) {
                primes[j] = false; // i의 배수를 모두 지움
            }
        }
    }
}
