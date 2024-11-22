fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut total = 0;

    nums.iter().map(|&num| {
        total += num;
        total
    }).collect()
}

fn main() {
    println!("{:?}", running_sum(vec![1, 1, 1, 1, 1]));
}
