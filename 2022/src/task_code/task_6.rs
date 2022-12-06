pub struct FifoBuffer{
    size: usize,
    buf: String,
}

impl FifoBuffer {
    pub fn new(size: i32) -> FifoBuffer {
        FifoBuffer { size: size as usize, buf: String::new()}
    }

    pub fn add(&mut self, input: char) -> () {
        if self.buf.len() == self.size {
            self.buf = String::from(&self.buf[1..]);
        } 
        self.buf.push(input);
    }

    pub fn is_unique(&self) -> bool {
        if self.buf.len() < self.size {
            return false;
        }
        let b = self.buf.as_bytes();
        for i in 0..self.size {
            let check = &b[i];
            let post = &b[i+1..];
            if post.contains(check) {
                return false;
            }
        }
        true
    }
}

fn internal(data:String, mut buf: FifoBuffer) -> i32 {
    for (i, char) in data.chars().enumerate() {
        buf.add(char);
        println!("{}", buf.buf);
        if buf.is_unique() {
            return (i+1) as i32
        }
    }

    0
}

pub fn part_1(data: String) -> i32 {
    let buf = FifoBuffer::new(4);
    internal(data, buf)
}

pub fn part_2(data: String) -> i32 {
    let buf = FifoBuffer::new(14);
    internal(data, buf)
}
