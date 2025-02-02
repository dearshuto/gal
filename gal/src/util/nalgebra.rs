use crate::kernels::{ICross, IDot, INorm, IVectorAccessorX, IVectorAccessorY};

impl<T: Clone> IVectorAccessorX<T> for nalgebra::Vector2<T> {
    fn x(&self) -> T {
        let v = &self.data.0[0];
        v[0].clone()
    }
}

impl<T: Clone> IVectorAccessorY<T> for nalgebra::Vector2<T> {
    fn y(&self) -> T {
        let v = &self.data.0[0];
        v[1].clone()
    }
}

impl<T> IDot<T> for nalgebra::Vector2<T>
where
    T: num::Zero + nalgebra::Scalar + nalgebra::ClosedAddAssign + nalgebra::ClosedMulAssign,
{
    fn dot(&self, lhs: &Self) -> T {
        self.dot(lhs)
    }
}

impl<T> INorm<T> for nalgebra::Vector2<T>
where
    T: nalgebra::SimdComplexField + nalgebra::SimdRealField,
{
    fn norm(&self) -> T {
        self.norm()
    }
}

impl<T> IDot<T> for nalgebra::Vector3<T>
where
    T: num::Zero + nalgebra::Scalar + nalgebra::ClosedAddAssign + nalgebra::ClosedMulAssign,
{
    fn dot(&self, lhs: &Self) -> T {
        self.dot(lhs)
    }
}

impl<T> ICross for nalgebra::Vector3<T>
where
    T: nalgebra::Scalar
        + nalgebra::ClosedAddAssign
        + nalgebra::ClosedMulAssign
        + nalgebra::ClosedSubAssign,
{
    fn cross(&self, other: &Self) -> Self {
        self.cross(other)
    }
}

impl<T> INorm<T> for nalgebra::Vector3<T>
where
    T: nalgebra::SimdComplexField + nalgebra::SimdRealField,
{
    fn norm(&self) -> T {
        self.norm()
    }
}
