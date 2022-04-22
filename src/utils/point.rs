use std::fmt;
#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
    pub fn x(self) -> usize {
        self.x
    }
    pub fn y(self) -> usize {
        self.y
    }

    fn read_value(
        input: &mut impl std::io::BufRead,
        value_name: &str,
    ) -> Result<usize, std::num::ParseIntError> {
        use std::io::Write;
        print!("{} - ", value_name);
        std::io::stdout().flush().unwrap();
        let mut buf = String::new();
        input.read_line(&mut buf).expect("A String");
        buf.trim().parse::<usize>()
        // usize::from_str_radix(&buf.trim(), 10)
    }

    pub fn from(input: &mut impl std::io::BufRead) -> Result<Point, std::num::ParseIntError> {
        let x = loop {
            match Point::read_value(input, "x") {
                Ok(value) => break value,
                Err(error) => println!("Error: \"{}\"", error),
            }
        };
        let y = loop {
            match Point::read_value(input, "y") {
                Ok(value) => break value,
                Err(error) => println!("Error: \"{}\"", error),
            }
        };
        Ok(Point { x, y })
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
