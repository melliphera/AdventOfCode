pub fn part_1(data: String) -> i32 {
    let mut accum = 0;
    let lines = data.split("\r\n");

    for line in lines {
        let nums = line.split(&['-', ','][..])
                       .map(|x| x.parse::<i32>().unwrap())
                       .collect::<Vec<i32>>();

        if nums[0] >= nums[2] && nums[1] <= nums[3] // first range inside second
            || nums[0] <= nums[2] && nums[1] >= nums[3] {
                
                accum += 1;
        }
    }

    accum
}

pub fn part_2(data: String) -> i32 {
    let mut accum = 0;
    let lines = data.split("\r\n");

    for line in lines {
        let nums = line.split(&['-', ','][..])
                       .map(|x| x.parse::<i32>().unwrap())
                       .collect::<Vec<i32>>();

        if nums[0] <= nums[2] && nums[2] <= nums[1]     // if left  starts lower
            || nums[0] >= nums[2] && nums[0] <= nums[3] // if right starts lower
            {    
                accum += 1;
        }
    }

    accum
}
