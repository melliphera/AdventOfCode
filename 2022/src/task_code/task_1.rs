pub fn part_1(raw: String) -> i32 {
    let mut max = 0;
    let mut accum: i32;

    let chunks = raw.split("\r\n\r\n");

    for chunk in chunks {
        accum = 0;
        for line in chunk.split_whitespace() {
            accum += line.parse::<i32>().unwrap()
        }
        if max < accum {
            max = accum
        }
    }
    
    max
}

pub fn part_2(raw: String) -> i32 {
    let mut greatest: Vec<i32> = vec!(0, 0, 0);
    let mut accum: i32;
    let mut buf: i32;

    let chunks = raw.split("\r\n\r\n");

    for chunk in chunks {
        accum = 0;
        for line in chunk.split_whitespace() {
            accum += line.parse::<i32>().unwrap()
        }
        
        let mut index = 0;
        loop {
            if greatest[index] < accum {
                buf = greatest[index];
                greatest[index] = accum;
                accum = buf;
            }

            index += 1;
            if index == greatest.len() {
                break
            }
        }
    }

    accum = 0;
    for i in 0..greatest.len() {
        accum += greatest[i]
    }
    println!("{:?}", greatest);
    accum
}