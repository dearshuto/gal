use super::{
    bezier_surface::BezierSurfaceBuilder, cube::CubeBuilder, sphere::SphereBuilder,
    triangulated_mesh::TriangulatedMesh,
};

pub trait IParametricBuilder {
    type Params;
    fn params<F>(self, func: F) -> Self
    where
        F: FnOnce(Self::Params) -> Self::Params;
}

pub trait ITriangulatedBuilder {
    fn triangulated(self) -> TriangulatedMesh;
}

pub struct Builder<T> {
    internal: T,
}

impl<T> Builder<T> {
    pub fn build(self) {}
}

impl<T: IParametricBuilder> Builder<T> {
    pub fn with_params<F>(mut self, func: F) -> Self
    where
        F: FnOnce(T::Params) -> T::Params,
    {
        self.internal = T::params(self.internal, func);
        self
    }
}

impl<T: ITriangulatedBuilder> Builder<T> {
    pub fn triangulated(self) -> TriangulatedMesh {
        self.internal.triangulated()
    }
}

impl Builder<CubeBuilder> {
    pub fn cube() -> Builder<CubeBuilder> {
        Self {
            internal: CubeBuilder::new(),
        }
    }
}

impl Builder<SphereBuilder> {
    pub fn sphere() -> Builder<SphereBuilder> {
        Self {
            internal: SphereBuilder {},
        }
    }
}

impl Builder<BezierSurfaceBuilder> {
    pub fn bezier() -> Builder<BezierSurfaceBuilder> {
        Self {
            internal: BezierSurfaceBuilder::new(),
        }
    }
}
