use num_traits::Float;

pub fn epsilon<T: Float>() -> T {
    T::from(0.0000000001).unwrap()
}

pub fn golden_ratio<T: Float>() -> T {
    (T::one() + T::sqrt(T::from(5).unwrap())) / T::from(2).unwrap()
}

pub fn minnick_a<T: Float>() -> T {
    T::from(54).unwrap().to_radians().sin() * (T::one() + T::from(72).unwrap().to_radians().cos())
}

pub fn minnick_b<T: Float>() -> T {
    T::one() / T::from(4).unwrap()
}

pub fn minnick_c<T: Float>() -> T {
    T::from(36).unwrap().to_radians().cos() - (T::from(3).unwrap() / T::from(4).unwrap())
}

pub fn minnick_d<T: Float>() -> T {
    minnick_a::<T>() - T::one()
}

pub fn minnick_e<T: Float>() -> T {
    golden_ratio::<T>() - minnick_b::<T>()
}

pub fn minnick_v<T: Float>() -> T {
    (T::one() / T::from(4).unwrap())
        * (T::from(3).unwrap() - (T::from(36).unwrap().tan() / T::from(18).unwrap().tan()))
}

pub fn minnick_w<T: Float>() -> T {
    T::from(3).unwrap() / T::from(4).unwrap()
}

pub fn minnick_x<T: Float>() -> T {
    T::one() + T::from(72).unwrap().to_radians().cos()
}

pub fn minnick_y<T: Float>() -> T {
    T::from(72).unwrap().to_radians().cos()
}

pub fn minnick_z<T: Float>() -> T {
    T::one() / T::from(4).unwrap()
}

pub fn scale<T: Float>() -> T {
    minnick_a::<T>() + minnick_w::<T>()
}

pub fn short<T: Float>() -> T {
    scale::<T>()
}

pub fn long<T: Float>() -> T {
    golden_ratio::<T>() * scale::<T>()
}

pub fn box_overlap<T: Float>() -> T {
    T::from(2.126627021).unwrap() // magic constant; is this δ?
}

pub fn box_dim<T: Float>() -> T {
    T::from(10).unwrap()
}

pub fn box_origin<T: Float>() -> T {
    -box_dim::<T>() / T::from(2).unwrap()
}
