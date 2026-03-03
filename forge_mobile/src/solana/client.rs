use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

#[derive(Serialize, Deserialize)]
struct AccountInfo {
    lamports: u64,
    owner: Pubkey,
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct TransactionStatus {
    slot: u64,
    confirmations: u64,
}

struct SolanaClient {
    client: RpcClient,
}

impl SolanaClient {
    pub fn new(url: &str) -> Self {
        SolanaClient { client: RpcClient::new(url.to_string()) }
    }

    pub fn get_account_info(&self, pubkey: &str) -> Result<AccountInfo> {
        let pubkey = Pubkey::from_str(pubkey).context("Failed to parse public key")?;
        let account = self.client.get_account(&pubkey).context("Failed to get account info")?;
        Ok(AccountInfo { lamports: account.lamports, owner: account.owner, data: account.data })
    }

    pub fn get_transaction_status(&self, signature: &str) -> Result<TransactionStatus> {
        let status = self.client.get_signature_status(signature).context("Failed to get transaction status")?;
        Ok(TransactionStatus { slot: status.slot, confirmations: status.confirmation })
    }

    pub fn get_sol_balance(&self, pubkey: &str) -> Result<u64> {
        let pubkey = Pubkey::from_str(pubkey).context("Failed to parse public key")?;
        let balance = self.client.get_balance(&pubkey).context("Failed to get SOL balance")?;
        Ok(balance)
    }

    pub fn get_token_supply(&self, mint_address: &str) -> Result<u64> {
        let mint_pubkey = Pubkey::from_str(mint_address).context("Failed to parse mint address")?;
        let supply = self.client.get_token_supply(&mint_pubkey).context("Failed to get token supply")?;
        Ok(supply.amount)
    }

    pub fn get_cluster_info(&self) -> Result<String> {
        let info = self.client.get_cluster_info().context("Failed to get cluster info")?;
        Ok(serde_json::to_string(&info).context("Failed to serialize cluster info")?)
    }

    pub fn get_recent_transactions(&self, pubkey: &str) -> Result<Vec<String>> {
        let pubkey = Pubkey::from_str(pubkey).context("Failed to parse public key")?;
        let transactions = self.client.get_confirmed_signature_for_address2(&pubkey, None).context("Failed to get recent transactions")?;
        Ok(transactions.into_iter().map(|tx| tx.signature).collect())
    }
}
