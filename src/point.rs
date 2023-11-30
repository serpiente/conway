use std::num::TryFromIntError;

#[derive(Debug, Copy, Clone)]
pub struct Point{
    pub x: usize,
    pub y: usize,
}

impl Point{

    pub fn new(x: usize, y:usize) -> Self {
        Self { x, y}
    }
}


impl TryFrom<(isize, isize)> for Point{
    type Error = TryFromIntError;

    fn try_from(value: (isize, isize)) -> Result<Self, Self::Error> {
        let x = value.0.try_into()?;
        let y = value.1.try_into()?;
        Ok(Self { x, y })
    }
}

impl From<(usize,usize)> for Point{
    fn from(value: (usize, usize)) -> Self {
        Self{ x:value.0, y:value.1 }
    }
}