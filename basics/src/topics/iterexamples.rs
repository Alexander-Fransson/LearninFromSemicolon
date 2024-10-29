#![allow(dead_code)]

fn iter_example() {
    // use iter when you want the values not to be consumed

    let numbers = vec![1, 2, 3];

    for num in numbers.iter() {
        println!("{}", num);
    }

    println!("the vector is {:?}", numbers);
}

fn into_iter_example() {
    // use into_iter when you want to consume the values
    let numbers = vec![1, 2, 3];

    // ownership is transferred
    for num in numbers.into_iter() {
        println!("{}", num);
    }
}

fn iter_mut_example() {
    // use iter_mut when you want to mutate the values
    let mut numbers = vec![1, 2, 3];

    println!("the vector before mutation is: {:?}", numbers);

    for num in numbers.iter_mut() {
        *num += 1;
    }

    println!("the vector is now: {:?}", numbers);
}