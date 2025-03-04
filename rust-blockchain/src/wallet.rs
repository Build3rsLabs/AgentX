use crate::error::{AppError, AppResult};
use bip39::{Mnemonic, Language};
use ed25519_dalek::{Keypair, SecretKey, PublicKey, Signer};
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};

pub struct Wallet {
    keypair: Keypair,
    mnemonic: Option<String>,
}

impl Wallet {
    pub fn generate() -> AppResult<Self> {
        let mut csprng = OsRng;
        
        // Generate a new keypair
        let keypair = Keypair::generate(&mut csprng);
        
        // Generate a mnemonic (BIP39)
        let mut entropy = [0u8; 32];
        rand::RngCore::fill_bytes(&mut csprng, &mut entropy);
        let mnemonic = Mnemonic::from_entropy(&entropy, Language::English)
            .map_err(|e| AppError::Wallet(format!("Failed to generate mnemonic: {}", e)))?;
        
        Ok(Self {
            keypair,
            mnemonic: Some(mnemonic.phrase().to_string()),
        })
    }
    
    pub fn from_mnemonic(phrase: &str) -> AppResult<Self> {
        let mnemonic = Mnemonic::parse_in(Language::English, phrase)
            .map_err(|e| AppError::Wallet(format!("Invalid mnemonic: {}", e)))?;
        
        let seed = mnemonic.to_seed("");
        
        // Use the first 32 bytes of the seed as the private key
        let secret = SecretKey::from_bytes(&seed[0..32])
            .map_err(|e| AppError::Wallet(format!("Failed to create secret key: {}", e)))?;
        
        let public = PublicKey::from(&secret);
        let keypair = Keypair { secret, public };
        
        Ok(Self {
            keypair,
            mnemonic: Some(phrase.to_string()),
        })
    }
    
    pub fn from_private_key(private_key: &[u8]) -> AppResult<Self> {
        let secret = SecretKey::from_bytes(private_key)
            .map_err(|e| AppError::Wallet(format!("Invalid private key: {}", e)))?;
        
        let public = PublicKey::from(&secret);
        let keypair = Keypair { secret, public };
        
        Ok(Self {
            keypair,
            mnemonic: None,
        })
    }
    
    pub fn address(&self) -> String {
        // MultiversX addresses start with "erd1"
        let mut hasher = Sha256::new();
        hasher.update(self.keypair.public.as_bytes());
        let hash = hasher.finalize();
        
        // Use the first 20 bytes of the hash for the address
        let address_bytes = &hash[0..20];
        
        // Encode with bech32 (simplified here - in a real implementation, use a proper bech32 library)
        format!("erd1{}", hex::encode(address_bytes))
    }
    
    pub fn sign_message(&self, message: &[u8]) -> Vec<u8> {
        let signature = self.keypair.sign(message);
        signature.to_bytes().to_vec()
    }
    
    pub fn sign_transaction(&self, transaction_data: &[u8]) -> String {
        let signature = self.keypair.sign(transaction_data);
        general_purpose::STANDARD.encode(signature.to_bytes())
    }
    
    pub fn private_key(&self) -> Vec<u8> {
        self.keypair.secret.as_bytes().to_vec()
    }
    
    pub fn public_key(&self) -> Vec<u8> {
        self.keypair.public.as_bytes().to_vec()
    }
    
    pub fn mnemonic(&self) -> String {
        self.mnemonic.clone().unwrap_or_else(|| "No mnemonic available".to_string())
    }
}