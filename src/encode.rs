pub enum StrategyDataTypes {
    VecFloat32(Vec<f32>),
    VecInt8(Vec<i8>),
    VecInt16(Vec<i16>),
    VecInt32(Vec<i32>),
    VecString(Vec<String>),
}

pub trait Strategy {
    fn apply(&self) -> Result<StrategyDataTypes, String>;
}
