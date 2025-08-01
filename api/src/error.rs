use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum GaryError {
    #[error("The epoch has ended and needs reset")]
    NeedsReset = 0,
    #[error("The provided hash is invalid")]
    HashInvalid = 1,
    #[error("The provided hash did not satisfy the minimum required difficulty")]
    HashTooEasy = 2,
    #[error("The claim amount cannot be greater than the claimable rewards")]
    ClaimTooLarge = 3,
    #[error("The clock time is invalid")]
    ClockInvalid = 4,
    #[error("You are trying to submit too soon")]
    Spam = 5,
    #[error("The maximum supply has been reached")]
    MaxSupply = 6,
    #[error("The proof does not match the expected account")]
    AuthFailed = 7,
}

error!(GaryError);
