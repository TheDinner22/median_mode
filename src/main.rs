use std::collections::HashMap;

fn main() {
    let nums: [i32; 10] = [1, 43534, 2342, 23423, 1231, 6546, 86767, 123, 567, 234];
    let nums: [i32; 12] = [1,2,3,4,5,6,7,8,9,10,11, 12 ];

    let median = get_median(Vec::from(nums));

    println!("{median}");

    let mode = get_mode(Vec::from(nums));

    println!("{mode}");
}

fn get_median(mut nums: Vec<i32>) -> f32{
    // sort 
    nums.sort();

    // get middle index(s)
    let length = nums.len();

    if length == 0 {
        0 as f32
    }
    else if length == 1 {
        nums[0] as f32
    }
    else if length % 2 == 1 {
        let middle = length/2;
        nums[middle] as f32
    }
    else {
        let first_middle = length/2;
        (((nums[first_middle - 1] + nums[first_middle]) as f32) / 2.0) as f32
    }
}

fn get_mode(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    let mut biggest_key: i32 = nums[0];

    for num in nums {
        let val = map.entry(num).or_insert(0);
        *val += 1;

        let val = *val;

        // compare current val to current biggest key's value
        let biggest_val = map.get(&biggest_key).unwrap();

        if val >= *biggest_val {
            biggest_key = num;
        }
    }

    biggest_key
}