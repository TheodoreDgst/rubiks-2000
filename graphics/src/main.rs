fn main() {
    let mut a = 0;

    for (p, i) in [3, 1, 6, 2, 4, 0, 5, 7].iter().enumerate() {
        a += i * (8 as u32).pow(p as u32);
    }

    // 16_434_824 : base
    // 16_434_243 : U
    // 16_041_872 : L
    // 16_008_587 : UL
    // 392_952 b - L
    

    println!("{}", a);

    let mut v = vec![];
    for _ in 0..8 {
        v.push(a % 8);
        a /= 8;
    }

    dbg!(v);
}
