#[derive(Debug,PartialEq)]
pub struct Point<T> {
	pub x: T,
	pub y: T,
}

#[derive(Debug)]
pub struct Line<T>(pub Point<T>, pub Point<T>);

pub type Polygon<T> = Vec<Point<T>>;

pub type Shape<T> = Vec<Polygon<T>>;

pub type Skeleton<T> = Vec<Line<T>>;
