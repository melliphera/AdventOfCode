use std::{ops::Range,
          fmt::Display};

#[derive(Debug, Clone, Copy)]
pub struct BingoBoard {
    nums: [i32; 25]
} 

impl BingoBoard {
    pub fn from(in_str: &str) -> BingoBoard {
        let rows = in_str.split("\r\n");
        let nums: [i32; 25] =  rows.map(|x| x.split(" ")
                                                 .filter_map(|i|  i.parse::<i32>()
                                                            .ok())
                                                            .collect::<Vec<i32>>()

                                            )
                    .flatten()
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap();
        

        BingoBoard {
            nums: nums
        }
    }

    pub fn get_sum(self) -> i32 {
        self.nums.iter().fold(0, |acc, item| {
            if item == &-1 {acc}
            else {acc + item}
        })
    }

    pub fn guess(&mut self, guess: i32) -> Option<i32>{
        match self.nums.iter().position(|x| x == &guess) {
            Some(index) => {
                // set number to 0
                self.nums[index] = -1;

                // check if the row or column is now entirely 0s (ie bingo)
                let gen: Range<usize> = 0..5;
                let row_range = gen.clone().map(|x| x + index/5 * 5).collect::<Vec<usize>>();
                let col_range = gen.map(|x| x*5 + index % 5).collect::<Vec<usize>>();
                //println!("guess: {} index: {}", guess, index);
                //println!("{}", self);

                let row_sum = row_range.into_iter().fold(0, |acc: i32, ind:usize| {acc + self.nums[ind]});
                let col_sum = col_range.into_iter().fold(0, |acc: i32, ind:usize| {acc + self.nums[ind]});

                if row_sum == -5 || col_sum == -5 {
                    Some(self.clone().get_sum())
                } else {
                    None
                }
            }
            None => None
        }
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut payload = String::with_capacity(80);
        for i in 0..25 {
            if self.nums[i] == -1 {
                payload += &format!("{:>3}", "X")[..];

            }   
            else {
                payload += &format!("{:>3}", self.nums[i])[..];
            }
            if (i) % 5 == 4 {payload += "\n"}
        }
        write!(f, "{}", &payload[..])
    }
}

pub fn part_1(data: String) -> i32 {
    let mut d_parts = data.split("\r\n\r\n");

    let order = d_parts.next().unwrap();

    let boards_map = d_parts.map(|x| BingoBoard::from(x));
    let mut boards = boards_map.collect::<Vec<BingoBoard>>();

    for num in order.split(",").map(|x| x.parse::<i32>().unwrap()) {
        for board in boards.iter_mut() {
            match board.guess(num) {
                Some(sum) => {
                    println!("{}\n{}\n{}\n", board, sum, num);
                    return sum * num}
                None => {}
            }
        }
    }
    0
}


pub fn part_2(data: String) -> i32 {
    let mut d_parts = data.split("\r\n\r\n");

    let order = d_parts.next().unwrap();

    let boards_map = d_parts.map(|x| BingoBoard::from(x));
    let mut boards = boards_map.collect::<Vec<BingoBoard>>();

    let mut last = 0;

    let mut completed: Vec<usize> = Vec::new();

    for num in order.split(",").map(|x| x.parse::<i32>().unwrap()) {
        for (i, board) in boards.iter_mut().enumerate() {
            if !(completed.contains(&i)) {
                match board.guess(num) {
                    Some(sum) => {
                        println!("{}\n{}\n{}\n", board, sum, num);
                        last = sum * num;
                        completed.push(i)
                    }
                    None => {}
                }
            }
        }
    }
    last
}
