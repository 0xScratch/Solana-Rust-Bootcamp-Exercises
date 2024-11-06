fn reverse_an_array_1(arr: &mut [i32]) {
    arr.reverse();
}

fn reverse_an_array_2(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len / 2 {
        arr.swap(i, len - 1 - i);
    }
}

fn main() {
    let mut array1 = [1, 2, 3, 4,];
    let mut array2 = [5, 6, 7, 8,];

    reverse_an_array_1(&mut array1);
    reverse_an_array_2(&mut array2);

    println!("Array reversal using built-in reverse function: {:?}", array1); 
    println!("Array reversal using loops and swap function: {:?}", array2);
}
