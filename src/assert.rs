pub struct Assert<const COND: bool>;

pub trait True {}
pub trait False {}

impl True for Assert<true> {}
impl False for Assert<false> {}
