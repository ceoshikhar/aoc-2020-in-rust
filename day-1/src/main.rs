use std::fs::File;
use std::io::prelude::*;

// As the "puzzle input" is a list of numbers in a text file seperated by
// `\n` new line char instead of an array of number seperated by ",", I have
// save those numbers in the file called `items.txt` and we are going to
// read the file and generate an array of numbers from the file so that we
// need an array of number to do out logic of solving this puzzle.
fn main() -> std::io::Result<()> {
    // Read the file
    let mut file = File::open("input.txt")?;

    // Contents of the file
    let mut contents = String::new();

    // We will store those numbers in this vector
    let mut array_of_num: Vec<i32> = vec![];

    // We store the chracters in this variable and then convert it to i32 when
    // we reach a new line char (`\n`)
    let mut num = String::from("");

    // Read the file and store the content
    file.read_to_string(&mut contents)?;

    // Iterator over the contents character by character
    for (_index, char) in contents.chars().enumerate() {
        if char == '\n' {
            // We convert the string of numbers in `num` to i32 so that we can
            // push it to `array_of_num`
            let num_int: i32 = num.parse().unwrap();
            array_of_num.push(num_int);
            // Clear the `num` string to store the next number string
            num.clear();
        } else {
            num.push(char);
        }
    }

    // At this point, we have all the numbers in the `array_of_num` to do our
    // main logic to solve the puzzle
    // println!("Array of numbers: {:#?}", array_of_num);

    // To store our puzzle's solution's value
    let mut multiplied_val: i32 = 0;

    // Iterate over the array of numbers and find those 2 numbers that sum to
    // `2020` and then we multiply those 2 numbers and that would be our
    // solution to the 1st puzzle
    for num1 in &array_of_num {
        for num2 in &array_of_num {
            if num1 + num2 == 2020 {
                multiplied_val = num1 * num2;
                break;
            }
        }
    }

    println!("The answer to 1st puzzle is: {}", multiplied_val);

    // Iterate over the array of numbers and find those 3 numbers that sum to
    // `2020` and then we multiply those 3 numbers and that would be our
    // solution to the 2nd puzzle
    for num1 in &array_of_num {
        for num2 in &array_of_num {
            for num3 in &array_of_num {
                if num1 + num2 + num3 == 2020 {
                    multiplied_val = num1 * num2 * num3;
                    break;
                }
            }
        }
    }

    println!("The answer to 2nd puzzle is: {}", multiplied_val);

    Ok(())
}
