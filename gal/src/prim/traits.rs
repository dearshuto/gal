pub trait IVerticesWriter<T>
where
    T: num::Float,
{
    fn write(&mut self, data: &[T]);
}

pub trait IIndicesWriter<T>
where
    T: num::Unsigned,
{
    fn write(&mut self, data: &[T]);
}
