use solana_program::pubkey::Pubkey;

/// The authority allowed to initialize the program.
pub const INITIALIZER_ADDRESS: Pubkey = Pubkey::from_str_const("joe4nk6iJZweJGmpyhRxYG3QGTqaUC3qKtbmxAJ9pwX");

/// The seed of the boost PDA.
pub const BOOST: &[u8] = b"boost";

/// The seed of the config PDA.
pub const CONFIG: &[u8] = b"config";

/// The seed of the stake PDA.
pub const STAKE: &[u8] = b"stake";

/// Denominator for basis point calculations.
pub const DENOMINATOR_BPS: u64 = 10_000;

/// The duration of a boost rotation in seconds.
pub const ROTATION_DURATION: i64 = 90;
