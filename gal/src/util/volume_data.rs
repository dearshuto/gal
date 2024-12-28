use num_traits::Unsigned;

use crate::traits::IVolumeData;

pub struct VolumeData<T, const TRESOLUTION: usize>
where
    T: Unsigned + Copy,
{
    internal: Vec<T>,
}

impl<T, const TRESOLUTION: usize> VolumeData<T, TRESOLUTION>
where
    T: Unsigned + Copy,
{
    pub fn new() -> Self {
        let size = TRESOLUTION * TRESOLUTION * TRESOLUTION;
        let mut internal = Vec::default();
        internal.resize(size, T::zero());
        Self { internal }
    }

    pub fn set(&mut self, x: u32, y: u32, z: u32, value: T) {
        let index =
            (x as usize) + TRESOLUTION * (y as usize) + TRESOLUTION * TRESOLUTION * (z as usize);
        self.internal[index] = value;
    }
}

impl<T: Unsigned + Copy, const TRESOLUTION: usize> IVolumeData<T> for VolumeData<T, TRESOLUTION> {
    fn get(&self, x: usize, y: usize, z: usize) -> T {
        let index =
            (x as usize) + TRESOLUTION * (y as usize) + TRESOLUTION * TRESOLUTION * (z as usize);
        self.internal[index]
    }

    fn width(&self) -> usize {
        TRESOLUTION
    }

    fn height(&self) -> usize {
        TRESOLUTION
    }

    fn depth(&self) -> usize {
        TRESOLUTION
    }
}
