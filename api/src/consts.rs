use array_const_fn_init::array_const_fn_init;
use const_crypto::ed25519;
use solana_program::{pubkey, pubkey::Pubkey};

/// The authority allowed to initialize the program.
pub const INITIALIZER_ADDRESS: Pubkey =
    Pubkey::from_str_const("joe4nk6iJZweJGmpyhRxYG3QGTqaUC3qKtbmxAJ9pwX");

/// The percentage denominator for the fee percentages.
pub const FEE_PERCENT_DENOMINATOR: u64 = 1000;

/// The address for receiving the taxes fee.
pub const TAXES_ADDRESSES: Pubkey =
    Pubkey::from_str_const("taxUg5xvdRz7Hc6Ps9fvNY5Coe1RxxH43cYnMHGHyaK");
pub const TAXES_PERCENT: u64 = 75; // 7.5%

/// The address for receiving the fines fee.
pub const FINES_ADDRESSES: Pubkey =
    Pubkey::from_str_const("Fine8MsMhoc5SaW9TjWzCc3RRDt6xD8m7RfTdrzaKLeP");
pub const FINES_PERCENT: u64 = 75; // 7.5%

/// The base reward rate to intialize the program with.
pub const INITIAL_BASE_REWARD_RATE: u64 = BASE_REWARD_RATE_MIN_THRESHOLD;

/// The minimum allowed base reward rate, at which point the min difficulty should be increased
pub const BASE_REWARD_RATE_MIN_THRESHOLD: u64 = 2u64.pow(6);

/// The maximum allowed base reward rate, at which point the min difficulty should be decreased.
pub const BASE_REWARD_RATE_MAX_THRESHOLD: u64 = 2u64.pow(9);

/// The spam/liveness tolerance in seconds.
pub const TOLERANCE: i64 = 5;

/// The minimum difficulty to initialize the program with.
pub const INITIAL_MIN_DIFFICULTY: u32 = 1;

/// The decimal precision of the GARY token.
/// There are 100_000 indivisible units per GARY (called "grains").
pub const TOKEN_DECIMALS: u8 = 5;

/// One GARY token, denominated in indivisible units.
pub const ONE_GARY: u64 = 10u64.pow(TOKEN_DECIMALS as u32);

/// The duration of one minute, in seconds.
pub const ONE_MINUTE: i64 = 60;

/// The number of minutes in a program epoch.
pub const EPOCH_MINUTES: i64 = 15;

/// The duration of a program epoch, in seconds.
pub const EPOCH_DURATION: i64 = ONE_MINUTE * EPOCH_MINUTES;

/// The maximum token supply (87B).
pub const MAX_SUPPLY: u64 = ONE_GARY * 87_000_000_000;

/// The target quantity of GARY to be mined per day.
pub const TARGET_DAY_REWARDS: u64 = ONE_GARY * 119_000_000;

/// The target quantity of GARY to be mined per minute.
pub const TARGET_MINUTE_REWARDS: u64 = TARGET_DAY_REWARDS / 24 / 60;

/// The target quantity of GARY to be mined per epoch.
pub const TARGET_EPOCH_REWARDS: u64 = TARGET_MINUTE_REWARDS * (EPOCH_MINUTES as u64);

/// The number of bus accounts, for parallelizing mine operations.
pub const BUS_COUNT: usize = 8;

/// The smoothing factor for reward rate changes. The reward rate cannot change by more or less
/// than a factor of this constant from one epoch to the next.
pub const SMOOTHING_FACTOR: u64 = 2;

/// The seed of the bus account PDA.
pub const BUS: &[u8] = b"bus";

/// The seed of the config account PDA.
pub const CONFIG: &[u8] = b"config";

/// The seed of the metadata account PDA.
pub const METADATA: &[u8] = b"metadata";

/// The seed of the mint account PDA.
pub const MINT: &[u8] = b"mint";

/// The seed of proof account PDAs.
pub const PROOF: &[u8] = b"proof";

/// The seed of the treasury account PDA.
pub const TREASURY: &[u8] = b"treasury";

/// Noise for deriving the mint pda
pub const MINT_NOISE: [u8; 16] = [
    46, 202, 60, 16, 79, 27, 56, 150, 214, 39, 251, 50, 182, 52, 153, 9,
];

/// The name for token metadata.
pub const METADATA_NAME: &str = "GARY";

/// The ticker symbol for token metadata.
pub const METADATA_SYMBOL: &str = "GARY";

/// The uri for token metdata.
pub const METADATA_URI: &str = "https://garygensler.org/metadata-v2.json";

/// Program id for const pda derivations
pub const PROGRAM_ID: [u8; 32] = unsafe { *(&crate::id() as *const Pubkey as *const [u8; 32]) };

/// The addresses of the bus accounts.
pub const BUS_ADDRESSES: [Pubkey; BUS_COUNT] = array_const_fn_init![const_bus_address; 8];

/// Function to derive const bus addresses.
const fn const_bus_address(i: usize) -> Pubkey {
    Pubkey::new_from_array(ed25519::derive_program_address(&[BUS, &[i as u8]], &PROGRAM_ID).0)
}

/// The address of the config account.
pub const CONFIG_ADDRESS: Pubkey =
    Pubkey::new_from_array(ed25519::derive_program_address(&[CONFIG], &PROGRAM_ID).0);

/// The address of the mint metadata account.
pub const METADATA_ADDRESS: Pubkey = Pubkey::new_from_array(
    ed25519::derive_program_address(
        &[
            METADATA,
            unsafe { &*(&mpl_token_metadata::ID as *const Pubkey as *const [u8; 32]) },
            unsafe { &*(&MINT_ADDRESS as *const Pubkey as *const [u8; 32]) },
        ],
        unsafe { &*(&mpl_token_metadata::ID as *const Pubkey as *const [u8; 32]) },
    )
    .0,
);

/// The address of the mint account: gary38zVE46XfdTZHa7QRwQGdJ9axXaE3WYt5U5K2C3
pub const MINT_ADDRESS: Pubkey =
    Pubkey::new_from_array(ed25519::derive_program_address(&[MINT, &MINT_NOISE], &PROGRAM_ID).0);

/// The bump of the mint account.
pub const MINT_BUMP: u8 = ed25519::derive_program_address(&[MINT, &MINT_NOISE], &PROGRAM_ID).1;

/// The address of the treasury account.
pub const TREASURY_ADDRESS: Pubkey =
    Pubkey::new_from_array(ed25519::derive_program_address(&[TREASURY], &PROGRAM_ID).0);

/// The bump of the treasury account, for cpis.
pub const TREASURY_BUMP: u8 = ed25519::derive_program_address(&[TREASURY], &PROGRAM_ID).1;

/// The address of the treasury token account.
pub const TREASURY_TOKENS_ADDRESS: Pubkey = Pubkey::new_from_array(
    ed25519::derive_program_address(
        &[
            unsafe { &*(&TREASURY_ADDRESS as *const Pubkey as *const [u8; 32]) },
            unsafe { &*(&spl_token::id() as *const Pubkey as *const [u8; 32]) },
            unsafe { &*(&MINT_ADDRESS as *const Pubkey as *const [u8; 32]) },
        ],
        unsafe { &*(&spl_associated_token_account::id() as *const Pubkey as *const [u8; 32]) },
    )
    .0,
);

/// The address of the CU-optimized Solana noop program.
pub const NOOP_PROGRAM_ID: Pubkey = pubkey!("noop8ytexvkpCuqbf6FB89BSuNemHtPRqaNC31GWivW");

#[cfg(test)]
mod tests {
    use crate::consts::{MINT, MINT_ADDRESS};
    use rand::{thread_rng, Rng};
    use solana_program::pubkey::Pubkey;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn gen_mint_noise() {
        let found = Arc::new(AtomicBool::new(false));
        let max_threads = 12; // 🔧 Change this to control number of CPU threads

        let mut handles = vec![];

        for _ in 0..max_threads {
            let found = Arc::clone(&found);

            let handle = thread::spawn(move || {
                let mut rng = thread_rng();

                while !found.load(Ordering::Relaxed) {
                    let noise: [u8; 16] = rng.gen();
                    let (pda, _) = Pubkey::find_program_address(&[MINT, &noise], &crate::id());
                    let pda_str = pda.to_string();

                    if pda_str.starts_with("gary") {
                        if !found.swap(true, Ordering::Relaxed) {
                            println!("\n✅ Found match!");
                            println!("MINT_NOISE: {:?}", noise);
                            println!("PDA:        {}", pda_str);
                        }
                        break;
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.join();
        }
    }

    #[test]
    fn t() {
        dbg!(MINT_ADDRESS);
    }
}
