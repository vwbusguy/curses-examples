#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[derive(Eq,PartialEq,Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct WinRef{
    pub ch: char,
    pub rtype: String,
}

