use serde_json::Value;

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(111754, output_part_1);
    assert_eq!(65402, output_part_2);
}

fn sum_numbers(json_struct: Value, ignore_reds: bool) -> i64 {
    match json_struct {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(vec) => vec
            .into_iter()
            .map(|val| sum_numbers(val, ignore_reds))
            .sum(),
        Value::Object(map) => {
            let mut total_sum = 0;
            for (_, val) in map {
                if let Value::String(s) = &val {
                    if ignore_reds && s == "red" {
                        return 0;
                    }
                }
                total_sum += sum_numbers(val, ignore_reds);
            }
            total_sum
        }
        _ => 0,
    }
}

fn part_1(input: &str) -> i64 {
    let parsed: Value = serde_json::from_str(input).unwrap();
    sum_numbers(parsed, false)
}

fn part_2(input: &str) -> i64 {
    let parsed: Value = serde_json::from_str(input).unwrap();
    sum_numbers(parsed, true)
}
