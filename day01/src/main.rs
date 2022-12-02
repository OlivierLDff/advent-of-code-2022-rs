use std::fs;
fn main() {
    let data = fs::read_to_string("./input.txt").unwrap();
    println!("Read input {}", &data);
    println!("Calories: {}", find_max_calories(&data));
    println!("3 max calories: {}", find_3_max_calories(&data));
}

fn create_list_of_calories_per_elf(data: &str) -> Vec<i32> {
    // get a string per elf calories
    // create a list of string per split
    // parse them to i32
    // accumulate them
    // Create a list of calories per elf
    data.replace('\r', "")
        .split("\n\n")
        .map(|elf_calories| elf_calories.split("\n"))
        .map(|calory| {
            calory
                .filter(|&calory| !calory.is_empty())
                .map(|calory_str| calory_str.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect()
}

fn find_max_calories(data: &str) -> i32 {
    create_list_of_calories_per_elf(data)
        .into_iter()
        .max()
        .unwrap()
}

fn find_3_max_calories(data: &str) -> i32 {
    let mut calories = create_list_of_calories_per_elf(data);
    calories.sort();
    calories[calories.len() - 3..calories.len()]
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let result = super::find_max_calories(
            "1000
2000\r\n3000

4000

5000
6000

7000
8000
9000

10000

",
        );

        assert_eq!(result, 24000)
    }

    #[test]
    fn test_part2() {
        let result = super::find_3_max_calories(
            "1000
2000\r\n3000

4000

5000
6000

7000
8000
9000

10000

",
        );

        assert_eq!(result, 45000)
    }
}
