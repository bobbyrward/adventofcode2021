use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T> Point<T>
where
    T: Copy,
{
    #[allow(dead_code)]
    pub fn as_tuple(&self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T> ops::Add<Point<T>> for Point<T>
where
    T: ops::Add<T>,
{
    type Output = Point<<T as ops::Add<T>>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> ops::Add<T> for Point<T>
where
    T: ops::Add<T> + Copy,
{
    type Output = Point<<T as ops::Add<T>>::Output>;

    fn add(self, rhs: T) -> Self::Output {
        Self::Output::new(self.x + rhs, self.y + rhs)
    }
}

impl<T> ops::AddAssign<Point<T>> for Point<T>
where
    T: ops::AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> ops::AddAssign<T> for Point<T>
where
    T: ops::AddAssign<T> + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl<T> ops::Sub<Point<T>> for Point<T>
where
    T: ops::Sub<T>,
{
    type Output = Point<<T as ops::Sub<T>>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> ops::Sub<T> for Point<T>
where
    T: ops::Sub<T> + Copy,
{
    type Output = Point<<T as ops::Sub<T>>::Output>;

    fn sub(self, rhs: T) -> Self::Output {
        Self::Output::new(self.x - rhs, self.y - rhs)
    }
}

impl<T> ops::SubAssign<Point<T>> for Point<T>
where
    T: ops::SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> ops::SubAssign<T> for Point<T>
where
    T: ops::SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl<T> ops::Mul<Point<T>> for Point<T>
where
    T: ops::Mul<T>,
{
    type Output = Point<<T as ops::Mul<T>>::Output>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> ops::Mul<T> for Point<T>
where
    T: ops::Mul<T> + Copy,
{
    type Output = Point<<T as ops::Mul<T>>::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> ops::MulAssign<Point<T>> for Point<T>
where
    T: ops::MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> ops::MulAssign<T> for Point<T>
where
    T: ops::MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> ops::Div<Point<T>> for Point<T>
where
    T: ops::Div<T>,
{
    type Output = Point<<T as ops::Div<T>>::Output>;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T> ops::Div<T> for Point<T>
where
    T: ops::Div<T> + Copy,
{
    type Output = Point<<T as ops::Div<T>>::Output>;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs)
    }
}

impl<T> ops::DivAssign<Point<T>> for Point<T>
where
    T: ops::DivAssign<T>,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T> ops::DivAssign<T> for Point<T>
where
    T: ops::DivAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_create() {
        assert_eq!(Point::new(1, 2).as_tuple(), (1, 2));
        assert_eq!(Point::new(3, 5).as_tuple(), (3, 5));
        assert_eq!(Point::default().as_tuple(), (0, 0));
    }

    #[test]
    fn test_point_add() {
        assert_eq!(Point::new(1, 1) + Point::new(2, 3), Point::new(3, 4));
        assert_eq!(Point::new(2, 3) + 1, Point::new(3, 4));

        let mut p = Point::new(5, 6);
        p += Point::new(1, 2);
        assert_eq!(p, Point::new(6, 8));

        let mut p = Point::new(5, 6);
        p += 2;
        assert_eq!(p, Point::new(7, 8));
    }

    #[test]
    fn test_point_sub() {
        assert_eq!(Point::new(1, 1) - Point::new(2, 3), Point::new(-1, -2));
        assert_eq!(Point::new(2, 3) - 1, Point::new(1, 2));

        let mut p = Point::new(5, 6);
        p -= Point::new(1, 2);
        assert_eq!(p, Point::new(4, 4));

        let mut p = Point::new(5, 6);
        p -= 2;
        assert_eq!(p, Point::new(3, 4));
    }

    #[test]
    fn test_point_mul() {
        assert_eq!(Point::new(3, 4) * Point::new(2, 3), Point::new(6, 12));
        assert_eq!(Point::new(3, 4) * 2, Point::new(6, 8));

        let mut p = Point::new(5, 6);
        p *= Point::new(1, 2);
        assert_eq!(p, Point::new(5, 12));

        let mut p = Point::new(5, 6);
        p *= 2;
        assert_eq!(p, Point::new(10, 12));
    }
    #[test]
    fn test_point_div() {
        assert_eq!(Point::new(12, 9) / Point::new(2, 3), Point::new(6, 3));
        assert_eq!(Point::new(12, 9) / 3, Point::new(4, 3));

        let mut p = Point::new(5, 6);
        p /= Point::new(1, 2);
        assert_eq!(p, Point::new(5, 3));

        let mut p = Point::new(10, 6);
        p /= 2;
        assert_eq!(p, Point::new(5, 3));
    }
}
