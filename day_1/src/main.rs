fn main() {
    // Read input from input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();
    // split input when two newlines are found
    let lines: Vec<&str> = input.split("\n\n").collect();

    let mut max_calories = 0;
    let mut top_three = TopThree::new();

    // For each elf (line) parse the number and get the sum of calories its holding
    // If it's more than max calories, set max calories to that elves total
    for line in lines {
        // split line into words and conver to numbers
        let items: Vec<&str> = line.split_whitespace().collect();
        let calories: Vec<i32> = items.iter().map(|x| x.parse().unwrap()).collect();
        let sum_calories: i32 = calories.iter().sum();
        if sum_calories > max_calories {
            max_calories = sum_calories;
        }
        // check if sum of calories is in top three
        top_three.is_top_three(sum_calories);
    }

    println!("Max calories: {}", max_calories);
    println!(
        "Top three: {}, {}, {}",
        top_three.first, top_three.second, top_three.third
    );
    println!("Sum of top three: {}", top_three.sum());
}

struct TopThree {
    first: i32,
    second: i32,
    third: i32,
}

impl TopThree {
    fn new() -> TopThree {
        TopThree {
            first: 0,
            second: 0,
            third: 0,
        }
    }

    fn is_top_three(&mut self, num: i32) {
        if num > self.first {
            self.third = self.second;
            self.second = self.first;
            self.first = num;
        } else if num > self.second {
            self.third = self.second;
            self.second = num;
        } else if num > self.third {
            self.third = num;
        }
    }

    fn sum(&self) -> i32 {
        self.first + self.second + self.third
    }
}
