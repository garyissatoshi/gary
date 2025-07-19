use gary_drillx::HashDigest;
use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum GaryInstruction {
    // User
    Claim = 0,
    Close = 1,
    Mine = 2,
    Open = 3,
    Reset = 4,
    Update = 5,

    // Admin
    Initialize = 100,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Claim {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Close {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Mine {
    pub digest: HashDigest,
    pub nonce: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Open {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Reset {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Update {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {}

instruction!(GaryInstruction, Claim);
instruction!(GaryInstruction, Close);
instruction!(GaryInstruction, Mine);
instruction!(GaryInstruction, Open);
instruction!(GaryInstruction, Reset);
instruction!(GaryInstruction, Update);
instruction!(GaryInstruction, Initialize);
