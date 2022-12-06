pub fn part_1(data: String) -> i32 {
    let mut accum = 0;
    for line in data.split("\r\n") {
        match &line[0..3] {
            "A X" => {accum += 4}
            "B X" => {accum += 1}
            "C X" => {accum += 7}
            "A Y" => {accum += 8}
            "B Y" => {accum += 5}
            "C Y" => {accum += 2}
            "A Z" => {accum += 3}
            "B Z" => {accum += 9}
            "C Z" => {accum += 6}
            _ => println!("{}", &line[0..3])
        }
    }

    accum
}

pub fn part_2(data: String) -> i32 {
    let mut accum = 0;
    for line in data.split("\r\n") {
        match &line[0..3] {
            "A X" => {accum += 3}
            "B X" => {accum += 1}
            "C X" => {accum += 2}
            "A Y" => {accum += 4}
            "B Y" => {accum += 5}
            "C Y" => {accum += 6}
            "A Z" => {accum += 8}
            "B Z" => {accum += 9}
            "C Z" => {accum += 7}
            _ => println!("{}", &line[0..3])
        }
    }

    accum
}