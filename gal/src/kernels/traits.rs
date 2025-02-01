pub trait IPoint2<T> {
    fn new(x: T, y: T) -> Self;
}

pub trait IPoint3<T> {
    fn new(x: T, y: T, z: T) -> Self;
}

pub trait IVector2<T> {
    fn new(x: T, y: T) -> Self;
}

pub trait IVector3<T> {
    fn new(x: T, y: T, z: T) -> Self;
}

pub trait IVectorAccessorX<T: Clone> {
    fn x(&self) -> T;
}

pub trait IVectorAccessorY<T: Clone> {
    fn y(&self) -> T;
}

pub trait IVectorAccessorZ<T: Clone> {
    fn z(&self) -> T;
}

pub trait IDot<T> {
    fn dot(&self, lhs: &Self) -> T;
}

pub trait ICross {
    fn cross(&self, other: &Self) -> Self;
}

pub trait INorm<T> {
    fn norm(&self) -> T;
}

pub trait IArcCos<T> {
    fn acos(self) -> T;
}

impl IArcCos<f32> for f32 {
    fn acos(self) -> f32 {
        self.acos()
    }
}

impl IArcCos<f64> for f64 {
    fn acos(self) -> f64 {
        self.acos()
    }
}
