fn main() {
    let gen_a_start = 699;
    let gen_b_start = 124;

    // let gen_a_start = 65;
    // let gen_b_start = 8921;

    // part1(gen_a_start, gen_b_start)
    part2(gen_a_start, gen_b_start);
}

fn part1(gen_a_start: u64, gen_b_start: u64) {
    let mut gen_a = gen_a_start;
    let mut gen_b = gen_b_start;

    let mut res = 0;
    for i in 0..40_000_000 {
        if i % 1_000_000 == 0 {
            println!("At iteration {i}");
        }
        gen_a = (gen_a * 16807) % 2147483647;
        gen_b = (gen_b * 48271) % 2147483647;

        if (gen_a as u16) == (gen_b as u16) {
            res += 1;
        }
    }
    println!("{res}")
}

fn part2(gen_a_start: u64, gen_b_start: u64) {
    let mut gen_a = gen_a_start;
    let mut gen_b = gen_b_start;

    let mut res = 0;
    for i in 0..5_000_000 {
        if i % 100_000 == 0 {
            println!("At iteration {i}");
        }
        loop {
            gen_a = (gen_a * 16807) % 2147483647;
            if gen_a % 4 == 0 {
                break;
            }
        }
        loop {
            gen_b = (gen_b * 48271) % 2147483647;
            if gen_b % 8 == 0 {
                break;
            }
        }

        if (gen_a as u16) == (gen_b as u16) {
            res += 1;
        }
    }
    println!("{res}")
}
