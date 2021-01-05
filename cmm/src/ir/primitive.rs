#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Primitive {
    Bool(bool),
    Float32(f32),
    Float64(f64),
    Integer8(u8),
    Integer32(u32),
    Integer64(u64),
    PointerInteger(u64),
}

impl From<bool> for Primitive {
    fn from(bool: bool) -> Self {
        Self::Bool(bool)
    }
}

impl From<f32> for Primitive {
    fn from(number: f32) -> Self {
        Self::Float32(number)
    }
}

impl From<f64> for Primitive {
    fn from(number: f64) -> Self {
        Self::Float64(number)
    }
}

impl From<u64> for Primitive {
    fn from(number: u64) -> Self {
        Self::PointerInteger(number)
    }
}
