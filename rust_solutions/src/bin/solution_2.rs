fn main() {
    // Part 1:
    std::fs::read_to_string("input2.txt")
        .map(|input| {
            let mut result: u64 = 0;

            input.split(',').for_each(|id| {
                if let Some((f, l)) = id.trim().split_once('-') {
                    let start: u64 = f.parse().unwrap();
                    let end: u64 = l.parse().unwrap();
                    for value in start..=end {
                        let val_string = value.to_string();
                        if val_string.len() % 2 == 0 {
                            let (a, b) = val_string.split_at(val_string.len() / 2);
                            if a == b {
                                result += value
                            };
                        }
                    }
                }
            });

            println!("Part 1: {:?}", result);
        })
        .unwrap();

    // Part 2:
    std::fs::read_to_string("input2.txt")
        .map(|input| {
            let mut result: u64 = 0;

            input.split(',').for_each(|id| {
                if let Some((f, l)) = id.trim().split_once('-') {
                    let start: u64 = f.parse().unwrap();
                    let end: u64 = l.parse().unwrap();
                    for value in start..=end {
                        let val_string = value.to_string();

                        for idx in 1..=val_string.len() / 2 {
                            let (a, b) = val_string.split_at(idx);
                            // Performance horror show but trivially easy to do
                            if b.replace(a, "").is_empty() {
                                result += value;
                                break;
                            }
                        }
                    }
                }
            });

            println!("Part 2: {:?}", result);
        })
        .unwrap();
}
