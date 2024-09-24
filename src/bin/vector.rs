// const my_numbers = vec![1, 2, 3, 4];

// let mut anothernumbers = Vec::new();

// anothernumbers.push(1);
// anothernumbers.push(6);

// anothernumbers.push(7);
// anothernumbers.pop();

// // gives you the length
// my_numbers.len();

// // slice
// let two = my_numbers[1];

struct Test {
    score: i32,
}

fn main() {
    // // iterating through a vector
    // for num in my_numbers {
    //     println("{:?}", num);
    // }

    // Example

    let my_scores = vec![
        Test { score: 99 },
        Test { score: 29 },
        Test { score: 40 },
        Test { score: 40 }
    ];

    for test in my_scores {
        println!("{:?}", test.score);
    }
}
