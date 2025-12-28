fn main() {
    // Part 1:
    std::fs::read_to_string("input3.txt")
        .map(|input| {
            let mut result: u64 = 0;

            input.split_ascii_whitespace().for_each(|bank| {
                let digits: Vec<u8> = bank.bytes().map(|b| b - b'0').collect();
                if let Some((elements, _)) = digits.split_last_chunk::<1>() {
                    let mut max: u8 = 0;
                    let mut idx: usize = 0;

                    for (k, &v) in elements.iter().enumerate() {
                        if v > max {
                            max = v;
                            idx = k;

                            if v == 9 {
                                break;
                            }
                        }
                    }

                    if let Some(m) = digits.iter().rev().take(digits.len() - idx - 1).max() {
                        let sum = max * 10 + m;
                        result += sum as u64;
                    }
                };
            });

            println!("Part 1: {:?}", result);
        })
        .unwrap();

    // Part 2:
    std::fs::read_to_string("input3.txt")
        .map(|input| {
            let mut result: u64 = 0;

            input.split_ascii_whitespace().for_each(|bank| {
                let digits: Vec<u8> = bank.bytes().map(|b| b - b'0').collect();
                let mut joltage: u64 = 0;
                let mut idx: usize = 0;

                for i in (0..=11).rev() {
                    if let Some((e, _)) = digits
                        .split_at_checked(idx)
                        .and_then(|(_, t)| t.split_at_checked(t.len() - i))
                    {
                        let mut t_max: u8 = 0;
                        let mut t_idx: usize = 0;

                        for (k, &v) in e.iter().enumerate() {
                            if v > t_max {
                                t_max = v;
                                t_idx = k;

                                if v == 9 {
                                    break;
                                }
                            }
                        }

                        idx += t_idx + 1;
                        joltage += t_max as u64 * 10_u64.pow(i as u32);
                    }
                }

                result += joltage;
            });

            println!("Part 2: {:?}", result);
        })
        .unwrap();
}
