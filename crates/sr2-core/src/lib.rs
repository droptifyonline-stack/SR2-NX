//! Renderer-independent prototype rules. No original game code or data.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Inventory {
    pub pink_slimes: u32,
}
impl Inventory {
    pub const CAPACITY: u32 = 20;
    pub fn capture(&mut self) -> bool {
        if self.pink_slimes >= Self::CAPACITY {
            return false;
        }
        self.pink_slimes += 1;
        true
    }
    pub fn release(&mut self) -> bool {
        if self.pink_slimes == 0 {
            return false;
        }
        self.pink_slimes -= 1;
        true
    }
}
/// Suction cone test; direction and offset are expressed in world coordinates.
pub fn suction_target(distance: f32, alignment: f32) -> bool {
    distance < 9.0 && alignment > 0.80
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inventory_limits_and_release() {
        let mut inventory = Inventory::default();
        assert!(!inventory.release());
        for _ in 0..20 {
            assert!(inventory.capture());
        }
        assert!(!inventory.capture());
        assert!(inventory.release());
        assert_eq!(inventory.pink_slimes, 19);
    }
    #[test]
    fn cone_rejects_outside_and_behind() {
        assert!(suction_target(5., 0.95));
        assert!(!suction_target(10., 1.));
        assert!(!suction_target(2., -1.));
    }
}
