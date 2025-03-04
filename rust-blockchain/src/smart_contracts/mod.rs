pub mod protocol_interface;
pub mod maiar_exchange;
pub mod hatom_protocol;
pub mod ashswap;

use crate::blockchain::MultiversXClient;
use crate::error::AppResult;
use crate::smart_contracts::protocol_interface::ProtocolInterface;
use crate::smart_contracts::maiar_exchange::MaiarExchangeProtocol;
use crate::smart_contracts::hatom_protocol::HatomProtocol;
use crate::smart_contracts::ashswap::AshSwapProtocol;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ProtocolRegistry {
    protocols: HashMap<String, Arc<dyn ProtocolInterface + Send + Sync>>,
}

impl ProtocolRegistry {
    pub fn new(blockchain_client: MultiversXClient) -> Self {
        let mut registry = Self {
            protocols: HashMap::new(),
        };
        
        // Register protocols
        registry.register_protocol(Arc::new(MaiarExchangeProtocol::new(blockchain_client.clone())));
        registry.register_protocol(Arc::new(HatomProtocol::new(blockchain_client.clone())));
        registry.register_protocol(Arc::new(AshSwapProtocol::new(blockchain_client.clone())));
        
        registry
    }
    
    pub fn register_protocol(&mut self, protocol: Arc<dyn ProtocolInterface + Send + Sync>) {
        self.protocols.insert(protocol.get_id().to_string(), protocol);
    }
    
    pub fn get_protocol(&self, id: &str) -> Option<Arc<dyn ProtocolInterface + Send + Sync>> {
        self.protocols.get(id).cloned()
    }
    
    pub fn get_all_protocols(&self) -> Vec<Arc<dyn ProtocolInterface + Send + Sync>> {
        self.protocols.values().cloned().collect()
    }
    
    pub async fn get_all_pools(&self) -> AppResult<HashMap<String, Vec<String>>> {
        let mut result = HashMap::new();
        
        for (id, protocol) in &self.protocols {
            let pools = protocol.get_pools().await?;
            result.insert(id.clone(), pools);
        }
        
        Ok(result)
    }
}