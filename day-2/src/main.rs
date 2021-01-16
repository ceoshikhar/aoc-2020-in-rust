use std::io;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    // File handler to read from the input
    let mut file = File::open("input.txt")?;

    // Buffer to store the input
    let mut input = String::new();

    // Read the input from the file and store it in the `contents` buffer string
    file.read_to_string(&mut input)?;

    // The input that is given to us to solve the puzzle is a simple string
    // in a form on very long string. To get to solving our puzzle we have to
    // first parse this input and form a Hash Map, the data structure which
    // makes sense to me and will help us start actually solving the puzzle.
    let input_hash_map: HashMap<usize, Password> = parse_input_to_hash_map(&input);
    let solution: usize = solve_puzzle(&input_hash_map);
    println!("The answer to 1st puzzel is: {}", &solution);

    Ok(())
}

// The password and the policy that was in effect when the password was created
#[derive(Debug)]
struct Password {
    // Minimum number of times the `letter` must be present in the `password`
    min_count:  u8,

    // Maximum number of times the `letter` can be present in the `password`
    max_count:  u8,

    // The letter that must be present in the `password`
    letter:     char,

    // The password that we want to check if it's valid according to the policy
    password:   String,
}

impl Password {
    fn new(min_count: u8,max_count: u8, letter: char, password: &str)
            -> Password {
       Password {
           min_count: min_count,
           max_count: max_count,
           letter: letter,
           password: password.to_string(),
       }
    }
}

// Generate the hash map from the raw input
fn parse_input_to_hash_map(input: &str) -> HashMap<usize, Password> {
    // We will parse the input and store the data in this hash map
    let mut hash_map: HashMap<usize, Password> = HashMap::new();

    // The key for every record in our hash map, which we increment after
    // we insert a record
    let mut key: usize      = 0;

    let mut min_count: u8   = 0;
    let mut max_count: u8   = 0;
    let mut letter: char    = ' ';
    let mut password        = String::new();

    // Split the input into lines
    for line in input.lines() {
        // Split each line by white space and put the items for each line in
        // a vector.
        //
        // This vector now has 3 key components for every record:
        //  - password
        //  - letter
        //  - min_count
        //  - max_count
        //
        // Sample `line_items` = ["qssqqxqqcqqgkzbq", "q:", "8-9"], where "q:"
        // contains our `letter`('q'), "8-9" contains our `min_count`(8) and
        // `max_count`(9) and the `password`("qssqqxqqcqqgkzbq")
        let line_items: Vec<&str> = line.rsplit(' ').collect();

        // We now need to just extract our data from each vector so that we can
        // form our Password instance and insert it in the `hash_map`. We start
        // by iterating of the items in the vector `line_items`
        for item in line_items {
            // From the sample `line_items` shown above we know our min and max
            // count values are in the item which contains '-'
            if item.contains('-') {
                let mut iter    = item.split(|c| c == '-');
                min_count       = iter.next().unwrap().parse().unwrap();
                max_count       = iter.next().unwrap().parse().unwrap();
            // From the sample `line_items` shown above we know our letter is
            // in the item which contains ':'
            } else if item.contains(':') {
                let mut iter    = item.split(|c| c == ':');
                letter          = iter.next().unwrap().parse().unwrap();
            } else {
                // The remaining item in the vector is the password itself
                password        = item.parse().unwrap();
            }
        }

        let password = Password::new(min_count, max_count, letter, &password);
        hash_map.insert(key, password);
        key += 1;
    }

    hash_map
}

// Find the number of password that are valid according to their policies
fn solve_puzzle(hash_map: &HashMap<usize, Password>) -> usize {
    let mut num_of_correct_passwords = 0;

    for val in hash_map.values() {
        // Num of times the `Password.letter` in appears in `Password.password`
        let mut num_of_times_letter_appears = 0;

        // Iterate over the `Password.password` char by char
        for letter in val.password.chars() {
            // If the current char in `Password.password` is same as
            // `Password.letter` we save the count of how many times
            // this appears to be true by incrementing
            // `num_of_times_letter_appaears`
            if letter == val.letter {
                num_of_times_letter_appears += 1;
            }
        }

        // Once we have iterated over the `Password.password` value, we check
        // if the number of occurrences of `Password.letter` is between
        // `Password.min_count` and `Password.max_count` inclusive of the range
        // or not. If it is then we increment the value of
        // `num_of_correct_passwords` as this is the check which tells us if the
        // `Password.password` is valid according to the password policy set for
        // that password.
        if num_of_times_letter_appears >= val.min_count
                && num_of_times_letter_appears <= val.max_count {
            num_of_correct_passwords += 1;
        }

    }

    return num_of_correct_passwords;
}
