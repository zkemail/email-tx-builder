use crate::*;
use ethers::middleware::Middleware;
use ethers::prelude::*;
use ethers::signers::Signer;

const CONFIRMATIONS: usize = 1;

type SignerM = SignerMiddleware<Provider<Http>, LocalWallet>;

#[derive(Debug, Clone)]
pub struct ChainClient {
    pub client: Arc<SignerM>,
    pub email_auth: EmailAuth<SignerM>,
    pub ecdsa_owned_dkim_registry: ECDSAOwnedDKIMRegistry<SignerM>,
}

impl ChainClient {
    pub async fn setup() -> Result<Self> {
        let wallet: LocalWallet = PRIVATE_KEY.get().unwrap().parse()?;
        let provider = Provider::<Http>::try_from(CHAIN_RPC_PROVIDER.get().unwrap())?;

        let client = Arc::new(SignerMiddleware::new(
            provider,
            wallet.with_chain_id(*CHAIN_ID.get().unwrap()),
        ));
        let email_auth = EmailAuth::new(
            CORE_CONTRACT_ADDRESS.get().unwrap().parse::<Address>()?,
            client.clone(),
        );
        let ecdsa_owned_dkim_registry =
            ECDSAOwnedDKIMRegistry::new(email_auth.dkim_registry_addr().await?, client.clone());

        Ok(Self {
            client,
            email_auth,
            ecdsa_owned_dkim_registry,
        })
    }

    pub fn self_eth_addr(&self) -> Address {
        self.client.address()
    }

    pub async fn set_dkim_public_key_hash(
        &self,
        selector: String,
        domain_name: String,
        public_key_hash: [u8; 32],
        signature: Bytes,
    ) -> Result<String> {
        // Mutex is used to prevent nonce conflicts.
        let mut mutex = SHARED_MUTEX.lock().await;
        *mutex += 1;

        let call = self.ecdsa_owned_dkim_registry.set_dkim_public_key_hash(
            selector,
            domain_name,
            public_key_hash,
            signature,
        );
        let tx = call.send().await?;
        let receipt = tx
            .log()
            .confirmations(CONFIRMATIONS)
            .await?
            .ok_or(anyhow!("No receipt"))?;
        let tx_hash = receipt.transaction_hash;
        let tx_hash = format!("0x{}", hex::encode(tx_hash.as_bytes()));
        Ok(tx_hash)
    }

    #[named]
    pub async fn check_if_dkim_public_key_hash_valid(
        &self,
        domain_name: ::std::string::String,
        public_key_hash: [u8; 32],
    ) -> Result<bool> {
        let is_valid = self
            .ecdsa_owned_dkim_registry
            .is_dkim_public_key_hash_valid(domain_name.clone(), public_key_hash)
            .call()
            .await?;
        info!(
            LOG,
            "{:?} for {} is already registered: {}", public_key_hash, domain_name, is_valid; "func" => function_name!()
        );
        Ok(is_valid)
    }

    pub async fn get_latest_block_number(&self) -> U64 {
        self.client.get_block_number().await.unwrap()
    }

    pub async fn is_wallet_deployed(&self, wallet_addr: &String) -> bool {
        // Check the bytecode of the contract
        let code = self.client.get_code(wallet_addr, None).await.unwrap();
        !code.is_empty()
    }

    pub async fn get_acceptance_subject_templates(
        &self,
        wallet_addr: &String,
        template_idx: u64,
    ) -> Result<Vec<String>, anyhow::Error> {
        let wallet_address: H160 = wallet_addr.parse()?;
        let contract = EmailAccountRecovery::new(wallet_address, self.client.clone());
        let templates = contract
            .acceptance_subject_templates()
            .call()
            .await
            .map_err(|e| anyhow::Error::from(e))?;
        Ok(templates[template_idx as usize].clone())
    }

    pub async fn get_recovery_subject_templates(
        &self,
        wallet_addr: &String,
        template_idx: u64,
    ) -> Result<Vec<String>, anyhow::Error> {
        let wallet_address: H160 = wallet_addr.parse()?;
        let contract = EmailAccountRecovery::new(wallet_address, self.client.clone());
        let templates = contract
            .recovery_subject_templates()
            .call()
            .await
            .map_err(|e| anyhow::Error::from(e))?;
        Ok(templates[template_idx as usize].clone())
    }

    pub async fn complete_recovery(&self, wallet_addr: &String) -> Result<bool, anyhow::Error> {
        let wallet_address: H160 = wallet_addr.parse()?;
        let contract = EmailAccountRecovery::new(wallet_address, self.client.clone());
        let call = contract.complete_recovery();
        let tx = call.send().await?;
        // If the transaction is successful, the function will return true and false otherwise.
        let receipt = tx
            .log()
            .confirmations(CONFIRMATIONS)
            .await?
            .ok_or(anyhow!("No receipt"))?;
        Ok(receipt
            .status
            .map(|status| status == U64::from(1))
            .unwrap_or(false))
    }

    pub async fn handle_acceptance(
        &self,
        wallet_addr: &String,
        email_auth_msg: EmailAuthMsg,
        template_idx: u64,
    ) -> Result<bool, anyhow::Error> {
        let wallet_address: H160 = wallet_addr.parse()?;
        let contract = EmailAccountRecovery::new(wallet_address, self.client.clone());
        let call = contract.handle_acceptance(email_auth_msg, template_idx.into());
        let tx = call.send().await?;
        // If the transaction is successful, the function will return true and false otherwise.
        let receipt = tx
            .log()
            .confirmations(CONFIRMATIONS)
            .await?
            .ok_or(anyhow!("No receipt"))?;
        Ok(receipt
            .status
            .map(|status| status == U64::from(1))
            .unwrap_or(false))
    }

    pub async fn handle_recovery(
        &self,
        wallet_addr: &String,
        email_auth_msg: EmailAuthMsg,
        template_idx: u64,
    ) -> Result<bool, anyhow::Error> {
        let wallet_address: H160 = wallet_addr.parse()?;
        let contract = EmailAccountRecovery::new(wallet_address, self.client.clone());
        let call = contract.handle_recovery(email_auth_msg, template_idx.into());
        let tx = call.send().await?;
        // If the transaction is successful, the function will return true and false otherwise.
        let receipt = tx
            .log()
            .confirmations(CONFIRMATIONS)
            .await?
            .ok_or(anyhow!("No receipt"))?;
        Ok(receipt
            .status
            .map(|status| status == U64::from(1))
            .unwrap_or(false))
    }
}
