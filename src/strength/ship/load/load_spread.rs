#[derive(PartialEq, Debug)]
pub enum LoadSpread {
    WithinOneSpatium,
    WithinManySpatiums,
    OutsideLeftmostFrame,
    OutsideRightmostFrame,
}