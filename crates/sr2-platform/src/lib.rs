//! Platform-neutral inputs; a native Horizon adapter remains unimplemented.
#[derive(Default, Clone, Copy, Debug)]
pub struct PlayerIntent {
    pub movement: [f32; 2],
    pub look: [f32; 2],
    pub jump: bool,
    pub suction: bool,
    pub launch: bool,
}
pub const TARGET_FRAME_SECONDS: f32 = 1.0 / 30.0;
pub const SWITCH_RENDERER_VERIFIED: bool = false;
