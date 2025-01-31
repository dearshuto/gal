use crate::kernels::{IDot, INorm};

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

impl<T> INorm<T> for nalgebra::Vector3<T>
where
    T: nalgebra::SimdComplexField + nalgebra::SimdRealField,
{
    fn norm(&self) -> T {
        self.norm()
    }
}
