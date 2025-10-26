use crate::prelude::*;
#[test]
#[should_panic]
fn test_arm() {
    let bomb = DropBomb::arm("hi :3");
    drop(bomb);
}
#[test]
fn test_defuse() {
    let mut bomb = DropBomb::arm("hi :3");
    bomb.defuse();
    drop(bomb);
}
