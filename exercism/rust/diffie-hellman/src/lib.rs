use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

fn modular_exponentiation(x: u64, e: u64, m: u64) -> u64 {
    if e == 0 {
        1
    } else if e == 1 {
        x
    } else if e % 2 == 0 {
        modular_exponentiation((x * x) % m, e / 2, m)
    } else {
        (x * modular_exponentiation((x * x) % m, (e - 1) / 2, m)) % m
    }
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub, a, p)
}
