fn main() {
    let dial_overflow: i32 = 100;
    let mut pos: i32 = 50;
    let mut crosses: i32 = 0;

    // Part 1:
    std::fs::read_to_string("input1.txt")
        .map(|input| {
            input.split_ascii_whitespace().for_each(|line| {
                let (s, r) = line.split_at(1);
                let sign: i32 = if s == "L" { -1 } else { 1 };
                let rot: i32 = r.parse().unwrap();
                pos += (rot % dial_overflow) * sign;

                if pos < 0 {
                    pos += dial_overflow;
                }

                if pos > 99 {
                    pos -= dial_overflow;
                }

                if pos == 0 {
                    crosses += 1;
                }
            })
        })
        .unwrap();

    println!("Part 1: {:?}", crosses);
    crosses = 0;

    // Part 2:
    std::fs::read_to_string("input1.txt")
        .map(|input| {
            input.split_ascii_whitespace().for_each(|line| {
                let (s, r) = line.split_at(1);
                let sign: i32 = if s == "L" { -1 } else { 1 };
                let rot: i32 = r.parse().unwrap();
                crosses += rot / dial_overflow;
                let change = (rot % dial_overflow) * sign;
                let prev_pos = pos;
                pos += change;

                if pos < 0 {
                    pos += dial_overflow;
                    if prev_pos != 0 {
                        crosses += 1;
                    }
                } else if pos > 99 {
                    pos -= dial_overflow;
                    if prev_pos != 0 {
                        crosses += 1;
                    }
                } else if pos == 0 {
                    crosses += 1;
                }
            })
        })
        .unwrap();

    println!("Part 2: {:?}", crosses);
}
