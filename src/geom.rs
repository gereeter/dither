use core::marker::PhantomData;

pub trait Affine3 {
    fn into_coords(self) -> [f64; 3];
    fn from_coords(coords: [f64; 3]) -> Self;
}

#[derive(Copy, Clone)]
pub struct Vec3<P> {
    pub coords: [f64; 3],
    _marker: PhantomData<P>
}

impl<P> core::ops::Add<Vec3<P>> for Vec3<P> {
    type Output = Vec3<P>;
    fn add(self, rhs: Vec3<P>) -> Vec3<P> {
        Vec3 { coords: [self.coords[0] + rhs.coords[0], self.coords[1] + rhs.coords[1], self.coords[2] + rhs.coords[2]], _marker: PhantomData }
    }
}

impl<P: Copy> core::ops::AddAssign<Vec3<P>> for Vec3<P> {
    fn add_assign(&mut self, rhs: Vec3<P>) {
        *self = *self + rhs;
    }
}

impl<P: Affine3> core::ops::Add<P> for Vec3<P> {
    type Output = P;
    fn add(self, rhs: P) -> P {
        let rhs_coords = rhs.into_coords();
        P::from_coords([self.coords[0] + rhs_coords[0], self.coords[1] + rhs_coords[1], self.coords[2] + rhs_coords[2]])
    }
}

impl<P> core::ops::Sub<Vec3<P>> for Vec3<P> {
    type Output = Vec3<P>;
    fn sub(self, rhs: Vec3<P>) -> Vec3<P> {
        Vec3 { coords: [self.coords[0] - rhs.coords[0], self.coords[1] - rhs.coords[1], self.coords[2] - rhs.coords[2]], _marker: PhantomData }
    }
}

impl<P> core::ops::Neg for Vec3<P> {
    type Output = Vec3<P>;
    fn neg(self) -> Vec3<P> {
        Vec3 { coords: [-self.coords[0], -self.coords[1], -self.coords[2]], _marker: PhantomData }
    }
}

impl<P> core::ops::Mul<f64> for Vec3<P> {
    type Output = Vec3<P>;
    fn mul(self, rhs: f64) -> Vec3<P> {
        Vec3 { coords: [self.coords[0] * rhs, self.coords[1] * rhs, self.coords[2] * rhs], _marker: PhantomData }
    }
}

impl<P> Vec3<P> {
    pub fn zero() -> Self {
        Vec3 { coords: [0.0, 0.0, 0.0], _marker: PhantomData }
    }

    pub fn dot(self, rhs: Vec3<P>) -> f64 {
        self.coords[0] * rhs.coords[0] + self.coords[1] * rhs.coords[1] + self.coords[2] * rhs.coords[2]
    }

    pub fn cross(self, rhs: Vec3<P>) -> Vec3<P> {
        Vec3 { coords: [
            self.coords[1]*rhs.coords[2] - rhs.coords[1]*self.coords[2],
            self.coords[2]*rhs.coords[0] - rhs.coords[2]*self.coords[0],
            self.coords[0]*rhs.coords[1] - rhs.coords[0]*self.coords[1]
        ], _marker: PhantomData }
    }
}

pub fn determinant<P>(values: [Vec3<P>; 3]) -> f64 {
    values[0].coords[0] * (values[1].coords[1] * values[2].coords[2] - values[1].coords[2] * values[2].coords[1]) +
    values[0].coords[1] * (values[1].coords[2] * values[2].coords[0] - values[1].coords[0] * values[2].coords[2]) +
    values[0].coords[2] * (values[1].coords[0] * values[2].coords[1] - values[1].coords[1] * values[2].coords[0])
}

pub fn subtract<P: Affine3>(to: P, from: P) -> Vec3<P> {
    Vec3 { coords: to.into_coords(), _marker: PhantomData } - Vec3 { coords: from.into_coords(), _marker: PhantomData }
}

pub fn midpoint<P: Affine3>(p1: P, p2: P) -> P {
    let p1_coords = p1.into_coords();
    let p2_coords = p2.into_coords();
    P::from_coords([
        (p1_coords[0] + p2_coords[0]) * 0.5,
        (p1_coords[1] + p2_coords[1]) * 0.5,
        (p1_coords[2] + p2_coords[2]) * 0.5,
    ])
}
