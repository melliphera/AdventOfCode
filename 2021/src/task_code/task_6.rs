#[allow(dead_code)]
struct NaiveBreeder {
    /// 9 stores, representing values 0 to 8 (read left to right)
    nums: [u64;9]
}

#[allow(dead_code)]
impl NaiveBreeder {
    pub fn new(input: String) -> NaiveBreeder {
        let payload = input
            .split(",")
            .map(|x| x.parse::<usize>().unwrap());

        let mut counter: [u64; 9] = [0,0,0,0,0,0,0,0,0];
        for num in payload {
            counter[num] += 1
        }
        let t = NaiveBreeder { nums: counter };
        t

    }

    pub fn pass_day(&mut self) {
        let zeros = self.nums[0];
        self.nums = [
            self.nums[1],         // new 0
            self.nums[2],         // new 1
            self.nums[3],         // new 2
            self.nums[4],         // new 3
            self.nums[5],         // new 4
            self.nums[6],         // new 5
            self.nums[7] + zeros, // new 6
            self.nums[8],         // new 7
            zeros                 // new 8
            ]
    }
}

struct SmartBreeder {
    nums: [u64;9],
    breed_pointer: usize
}

impl SmartBreeder {
    pub fn new(input: String) -> SmartBreeder {
        let payload = input
            .split(",")
            .map(|x| x.parse::<usize>().unwrap());

        let mut counter: [u64; 9] = [0,0,0,0,0,0,0,0,0];
        for num in payload {
            counter[num] += 1
        }
        let t = SmartBreeder { nums: counter, breed_pointer: 0 };
        t

    }
    
    pub fn pass_day(&mut self) {
        let (from, parent, ) = (self.breed_pointer, (self.breed_pointer + 7) % 9);
        self.nums[parent] =   self.nums[parent] + self.nums[from];
        self.breed_pointer = (self.breed_pointer + 1) % 9 
        
    }

    #[allow(dead_code)]
    pub fn shifted(&self) -> [u64; 9] {
        // returns an array in the same formate as NaiveBreeder, 
        // where the 0th index is always 0 days from breeding, 1st is 1 day away and so on.
        let mut post: Vec<u64> = self.nums[self.breed_pointer..].try_into().unwrap();
        let mut pre: Vec<u64> = self.nums[..self.breed_pointer].try_into().unwrap();

        post.append(&mut pre);
        post.try_into().unwrap()
    }
}

pub fn internal(day_max: usize, data: String) -> u64 {
    let mut sb = SmartBreeder::new(data);

    for _i in 0..day_max {
        sb.pass_day();
        // println!("{:?}", sb.nums)
    }
    sb.nums.iter().fold(0, |acc, x| acc + *x as u64)
}

pub fn part_1(data: String) -> u64 {  
    internal(80, data)
}

pub fn part_2(data: String) -> u64 {  
    internal(256, data)
}

