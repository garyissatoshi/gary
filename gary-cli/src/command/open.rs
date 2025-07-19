use crate::utils::ComputeBudget;
use crate::Miner;
use gary_api::prelude::proof_pda;
use solana_sdk::signature::Signer;

impl Miner {
    pub(crate) async fn open(&self) {
        // Register miner
        let mut ixs = Vec::new();
        let signer_pubkey = self.signer().pubkey();
        let fee_payer = self.fee_payer();
        let proof_address = proof_pda(signer_pubkey).0;
        if self.rpc_client.get_account(&proof_address).await.is_err() {
            let ix = gary_api::sdk::open(signer_pubkey, signer_pubkey, fee_payer.pubkey());
            ixs.push(ix);
        }

        // Submit transaction
        if !ixs.is_empty() {
            self.send_and_confirm(&ixs, ComputeBudget::Fixed(400_000), false)
                .await
                .ok();
        }
    }
}
