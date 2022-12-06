pub fn part_1(data: String) -> i32 {
    let mut accum = 0;
    let lines = data.split("\r\n");

    for line in lines {
        let b = line.as_bytes();
        let (left, right) = (&b[0..b.len()/2], &b[b.len()/2..]);
        for num in left {
            if right.contains(num) {
                    match num {
                        65..=90  => {accum += *num as i32 - 38}
                        97..=122 => {accum += *num as i32 - 96}
                        _ => unreachable!("Text must be in above defined number ranges.")
                    }
                break
            }
        }
    }

    accum
}

pub fn part_2(data: String) -> i32 {
    let mut accum = 0;
    let lines = data.split("\r\n").collect::<Vec<&str>>();

    for i in 0..lines.len()/3 {
        let chunk = &lines[3*i..3*(i+1)];
        for char in chunk[0].chars() {
             if chunk[1].contains(char) && chunk[2].contains(char) {
                let num = char as u8;
                match num  {
                    65..=90  => {accum += num as i32 - 38}
                    97..=122 => {accum += num as i32 - 96}
                    _ => unreachable!("Text must be in above defined number ranges.")
                }
                break
             }
        }
    }

    accum
}