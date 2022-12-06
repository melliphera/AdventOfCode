pub struct Line<T> {
    start_x: T,
    end_x:   T,
    start_y: T,
    end_y:   T,
}



impl Line<i32> {
    pub fn from_string(line:&str) -> Line<i32> {
        let sides = line.split(" -> ");
        let parts = sides
            .map(|x| x.split(","))
            .flatten()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();    

        Line {
            start_x: parts[0],
            end_x:   parts[2],
            start_y: parts[1],
            end_y:   parts[3],
        }
    }

    pub fn create_indices(&self) -> Vec<[i32; 2]> {
        let mut out: Vec<[i32; 2]> = Vec::new();
        // vertical case
        if self.start_x == self.end_x {
            if self.start_y < self.end_y {
                for i in self.start_y..=self.end_y {
                    out.push([self.start_x, i])
                }
            }
            else {
                for i in self.end_y..=self.start_y {
                    out.push([self.start_x, i])
                }
            }
        }
        
        // horizontal case 
        else if self.start_y == self.end_y {
            if self.start_x < self.end_x {
                for i in self.start_x..=self.end_x {
                    out.push([i, self.start_y])
                }
            }
            else {
                for i in self.end_x..=self.start_x {
                    out.push([i, self.start_y])
                }
            }
        } 

        // diagonal case
        else {

            // these two = 1 if respective end is higher number than start, -1 otherwise.
            let x_sign  = (self.end_x - self.start_x)/self.end_x.abs_diff(self.start_x) as i32; 
            let y_sign  = (self.end_y - self.start_y)/self.end_y.abs_diff(self.start_y) as i32; 
            for i in 0..=(self.end_x.abs_diff(self.start_x) as i32) {
                out.push([self.start_x + i*x_sign, self.start_y + i*y_sign])
            }
        }
        out
    }
}

pub fn part_1(data:String) -> i32 {
    internal(1, data)
}

pub fn part_2(data:String) -> i32 {
    internal(2, data)
}

fn internal(part: i32, data:String) -> i32 {
    let points_iter = data.lines()
            .map(|line| Line::from_string(line));

    // non-performant code - would read in place if performance was important
    let max_x = points_iter.clone()
                    .map(|line| [line.start_x, line.end_x])
                    .flatten()
                    .fold(0, |acc, x| {if x > acc  {x} else {acc}}); // max function

    // non-performant code - would read in place if performance was important
    let max_y = points_iter.clone()
                    .map(|line| [line.start_y, line.end_y])
                    .flatten()
                    .fold(0, |acc, y| {if y > acc  {y} else {acc}}); // max function

    let (straights, diags):(Vec<Line<i32>>, Vec<Line<i32>>) = points_iter.partition(
                    |line| {line.start_x == line.end_x || line.start_y == line.end_y}
                    ); // part 1 filter

    let mut board = vec!(vec!(0; (max_y+1) as usize); (max_x+1) as usize);  
    
    for row in straights.into_iter() {
        let indices = row.create_indices();
        println!("{:?}", indices);
        indices.into_iter().map(|[x, y]| board[y as usize][x as usize] += 1).for_each(drop);
    }
    println!("\n");
    
    if part == 2 {
        for row in diags.into_iter() {
            let indices = row.create_indices();
            println!("{:?}", indices);
            indices.into_iter().map(|[x, y]| board[y as usize][x as usize] += 1).for_each(drop);
        }
    }

    // counts how many instances of x > 1 on board, consuming it.
    board.into_iter()
         .flatten()
         .fold(0, |acc, x| {if x > 1 {acc + 1} else {acc}})
}