use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, i32>::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        println!("{}", complement);

        if map.contains_key(&complement) {
            return vec![map[&complement], i as i32];
        }

        map.insert(num, i as i32);
    }

    vec![]
}

fn main() {
    println!("Input: nums = {:?}, target = {}", vec![2, 3, 4, 5,], 9);
    println!("Output: {:?}", two_sum(vec![2, 3, 4, 5,], 9));
}
