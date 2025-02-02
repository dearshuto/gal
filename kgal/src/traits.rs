pub trait ILine<T: Clone> {
    fn a(&self) -> T;
    fn b(&self) -> T;
    fn c(&self) -> T;
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

pub trait ICompareX {
    type Type;

    fn has_greater_x_than(&self, other: &Self::Type) -> bool;
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

pub trait ISqrt<T> {
    fn sqrt(self) -> T;
}

pub trait IArcCos<T> {
    fn acos(self) -> T;
}
