use steel::*;

use super::GaryAccount;

/// Treasury is a singleton account which is the mint authority for the GARY token and the authority of
/// the program's global token account.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Treasury {}

account!(GaryAccount, Treasury);
