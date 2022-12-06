use std::{str,
          fmt::Display};

#[derive(Clone)]
pub struct Stack {
    boxes: String // Stored as string of letters, bottom to top
}

pub struct Warehouse {
    stacks: Vec<Stack>,
}

impl Warehouse {
    fn load_from_string(str: String) -> Warehouse {
        let mut lines = str.rsplit("\r\n");
        let count_line = lines.next().unwrap().as_bytes();
        let count = ((count_line.len()+1) / 4 ) as usize;

        let mut out = Warehouse {
            stacks: (0..count).map(|_| Stack {boxes:String::new(),}).collect(),
        };

        while let Some(line) = lines.next() {
            let l_bytes = line.as_bytes();
            for i in 0..count {
                let ind_array = [l_bytes[(4*i)+1]];
                let character = str::from_utf8(&ind_array).unwrap();
                if character != " " { out.stacks[i].boxes += character; }

            }
        }
        // println!("{}",out);
        out
    }

    fn execute_instructions_part(&mut self, part: i32, str:String) -> () {
        let instructions = str.split("\r\n");
        for instruction in instructions {
            let words = instruction.split(" ").collect::<Vec<&str>>();
            // println!("{} -> {}; {} boxes", words[3], words[5], words[1]);
            let numbers: [i32;3] = [words[3], words[5], words[1]].map(|x| x.parse().unwrap());
            
            let [donor, recipient, num_boxes] = numbers.map(|x| x as usize);
            let donor_stack = &mut self.stacks[donor-1];
            let payload: String;
            match  part {
                1 => { payload = donor_stack.boxes[donor_stack.boxes.len()-num_boxes..]
                                    .chars()
                                    .rev()
                                    .collect::<String>();
                    },
                2 => { payload = donor_stack.boxes[donor_stack.boxes.len()-num_boxes..].to_owned() }
                _ => unreachable!()

            }


            donor_stack.boxes = donor_stack.boxes[..donor_stack.boxes.len()-payload.len()].to_owned();
            let recipient_stack = &mut self.stacks[recipient-1];
            recipient_stack.boxes  += &payload[..];

            // println!("{}", self)

        }
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = &self.stacks.clone().into_iter().map(|x| x.boxes.len()).max().unwrap();
        let stack_count = self.stacks.len();
        let bytes = height * (stack_count * 4 + 1);
        let mut out: String = String::with_capacity(bytes);


        for i in 0..height*stack_count {                
            let stack = i % stack_count;  
            let box_string = &self.stacks[stack].boxes;              
            let ind = height - (i / stack_count + 1);  
            if ind < box_string.len() {
                out += &format!("[{}] ", &box_string[ind..ind+1])[..]
            } else {
                out += "    ";
            }
            if stack == stack_count -1 { out += "\n" }
        }

        for i in 1..=stack_count {
            out += &format!(" {:<3}", i)
        }

        write!(f, "{}", out)
    }
}

pub fn part_1(data: String) -> i32 {
    let layout = data.split("\r\n\r\n").collect::<Vec<&str>>();
    let arrangement = String::from(layout[0]);
    let mut stacks = Warehouse::load_from_string(arrangement);
    let instructions = String::from(layout[1]);
    stacks.execute_instructions_part(1, instructions);

    0
}

pub fn part_2(data: String) -> i32 {
    let layout = data.split("\r\n\r\n").collect::<Vec<&str>>();
    let arrangement = String::from(layout[0]);
    let mut stacks = Warehouse::load_from_string(arrangement);
    let instructions = String::from(layout[1]);
    stacks.execute_instructions_part(2, instructions);

    0
}

/*
pub fn part_2(data: String) -> i32 {
    let mut accum = 0;
    let lines = data.split("\r\n");

    accum
}
*/