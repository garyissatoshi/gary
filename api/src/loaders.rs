use steel::*;

use crate::{
    consts::*,
    state::{Config, Treasury},
};

pub trait GaryAccountInfoValidation {
    fn is_bus(&self) -> Result<&Self, ProgramError>;
    fn is_config(&self) -> Result<&Self, ProgramError>;
    fn is_treasury(&self) -> Result<&Self, ProgramError>;
    fn is_treasury_tokens(&self) -> Result<&Self, ProgramError>;
}

impl GaryAccountInfoValidation for AccountInfo<'_> {
    fn is_bus(&self) -> Result<&Self, ProgramError> {
        if !BUS_ADDRESSES.contains(self.key) {
            return Err(ProgramError::InvalidSeeds);
        }
        Ok(self)
    }

    fn is_config(&self) -> Result<&Self, ProgramError> {
        self.has_address(&CONFIG_ADDRESS)?
            .is_type::<Config>(&crate::ID)
    }

    fn is_treasury(&self) -> Result<&Self, ProgramError> {
        self.has_address(&TREASURY_ADDRESS)?
            .is_type::<Treasury>(&crate::ID)
    }

    fn is_treasury_tokens(&self) -> Result<&Self, ProgramError> {
        self.has_address(&TREASURY_TOKENS_ADDRESS)
    }
}
