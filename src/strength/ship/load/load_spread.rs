pub enum LoadSpread {
    WithinOneSpatium,
    WithinManySpatiums,
    // TODO: Объединить в одну OutsideOuterFrames.
    OutsideLeftmostFrame,
    OutsideRightmostFrame,
}