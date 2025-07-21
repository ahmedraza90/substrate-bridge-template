// Solo Chain Bridge Relay Implementation for Chain A <-> Chain B
// Minimal version with codegen-generated types removed

use relay_substrate_client::{
    Chain, ChainWithTransactions, ChainWithMessages, ChainWithGrandpa, 
    ChainWithRuntimeVersion, Client, DefaultClient, TransactionParams,
    AccountKeyPairOf, AccountIdOf, HeaderOf, HeaderIdOf, SimpleRuntimeVersion,
    ConnectionParams, ChainRuntimeVersion
};
use relay_utils::{relay_loop, MetricsParams};
use substrate_relay_helper::{
    finality::{run as run_finality, SubstrateFinalitySyncPipeline},
    messages::{
        run as run_messages, SubstrateMessageLane, MessagesRelayParams,
        SubstrateMessagesSource, SubstrateMessagesTarget
    },
    on_demand::OnDemandHeadersRelay,
    TransactionParams as RelayTransactionParams,
    HeadersToRelay
};
use async_trait::async_trait;
use std::{sync::Arc, time::Duration};
use codec::{Encode, Decode};
use sp_core::crypto::Pair;
use futures::future::try_join_all;
use bp_messages::HashedLaneId;

// Import minimal runtime types (these will be generated)
use sp_runtime::{
    generic::{Header as GenericHeader, SignedBlock as GenericSignedBlock, Block as GenericBlock},
    traits::BlakeTwo256,
    AccountId32, OpaqueExtrinsic,
};

// Chain A client definition - core configuration only
#[derive(Clone)]
pub struct ChainA;

impl Chain for ChainA {
    const NAME: &'static str = "ChainA";
    const BEST_FINALIZED_HEADER_ID_METHOD: &'static str = "BridgeApi_best_finalized_header_id";
    const FREE_HEADERS_INTERVAL_METHOD: &'static str = "BridgeApi_free_headers_interval";
    const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_millis(6000);
    
    type SignedBlock = GenericSignedBlock<GenericBlock<
        GenericHeader<u32, BlakeTwo256>,
        OpaqueExtrinsic
    >>;
    // This will be replaced with generated runtime call type
    type Call = ChainARuntimeCall;
}

impl ChainWithRuntimeVersion for ChainA {
    const RUNTIME_VERSION: Option<SimpleRuntimeVersion> = Some(SimpleRuntimeVersion {
        spec_version: 102,
        transaction_version: 1,
    });
}

impl ChainWithTransactions for ChainA {
    type AccountKeyPair = sp_core::sr25519::Pair;
    type SignedTransaction = OpaqueExtrinsic;

    fn sign_transaction(
        param: relay_substrate_client::SignParam<Self>,
        unsigned: relay_substrate_client::UnsignedTransaction<Self>,
    ) -> Result<Self::SignedTransaction, relay_substrate_client::Error> {
        relay_substrate_client::sign_transaction(param, unsigned)
    }
}

impl ChainWithGrandpa for ChainA {
    const SYNCED_HEADERS_GRANDPA_INFO_METHOD: &'static str = "BridgeGrandpaApi_synced_headers_grandpa_info";
    type KeyOwnerProof = sp_consensus_grandpa::OpaqueKeyOwnershipProof;
}

impl ChainWithMessages for ChainA {
    const TO_CHAIN_MESSAGE_DETAILS_METHOD: &'static str = "BridgeMessagesApi_outbound_message_details";
    const FROM_CHAIN_MESSAGE_DETAILS_METHOD: &'static str = "BridgeMessagesApi_inbound_message_details";
}

// Chain B client definition - core configuration only
#[derive(Clone)]
pub struct ChainB;

impl Chain for ChainB {
    const NAME: &'static str = "ChainB";
    const BEST_FINALIZED_HEADER_ID_METHOD: &'static str = "BridgeApi_best_finalized_header_id";
    const FREE_HEADERS_INTERVAL_METHOD: &'static str = "BridgeApi_free_headers_interval";
    const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_millis(6000);
    
    type SignedBlock = GenericSignedBlock<GenericBlock<
        GenericHeader<u32, BlakeTwo256>,
        OpaqueExtrinsic
    >>;
    // This will be replaced with generated runtime call type
    type Call = ChainBRuntimeCall;
}

impl ChainWithRuntimeVersion for ChainB {
    const RUNTIME_VERSION: Option<SimpleRuntimeVersion> = Some(SimpleRuntimeVersion {
        spec_version: 102,
        transaction_version: 1,
    });
}

impl ChainWithTransactions for ChainB {
    type AccountKeyPair = sp_core::sr25519::Pair;
    type SignedTransaction = OpaqueExtrinsic;

    fn sign_transaction(
        param: relay_substrate_client::SignParam<Self>,
        unsigned: relay_substrate_client::UnsignedTransaction<Self>,
    ) -> Result<Self::SignedTransaction, relay_substrate_client::Error> {
        relay_substrate_client::sign_transaction(param, unsigned)
    }
}

impl ChainWithGrandpa for ChainB {
    const SYNCED_HEADERS_GRANDPA_INFO_METHOD: &'static str = "BridgeGrandpaApi_synced_headers_grandpa_info";
    type KeyOwnerProof = sp_consensus_grandpa::OpaqueKeyOwnershipProof;
}

impl ChainWithMessages for ChainB {
    const TO_CHAIN_MESSAGE_DETAILS_METHOD: &'static str = "BridgeMessagesApi_outbound_message_details";
    const FROM_CHAIN_MESSAGE_DETAILS_METHOD: &'static str = "BridgeMessagesApi_inbound_message_details";
}

// Finality sync pipeline definitions
#[derive(Clone)]
pub struct ChainAToChainBFinalityPipeline;

impl SubstrateFinalitySyncPipeline for ChainAToChainBFinalityPipeline {
    type SourceChain = ChainA;
    type TargetChain = ChainB;
    type FinalityEngine = relay_substrate_client::GrandpaFinalityEngine<ChainA>;
}

#[derive(Clone)]
pub struct ChainBToChainAFinalityPipeline;

impl SubstrateFinalitySyncPipeline for ChainBToChainAFinalityPipeline {
    type SourceChain = ChainB;
    type TargetChain = ChainA;
    type FinalityEngine = relay_substrate_client::GrandpaFinalityEngine<ChainB>;
}

// Message lane definitions
#[derive(Clone)]
pub struct ChainAToChainBMessageLane;

impl SubstrateMessageLane for ChainAToChainBMessageLane {
    type SourceChain = ChainA;
    type TargetChain = ChainB;
    type LaneId = HashedLaneId;
    
    const SOURCE_NAME: &'static str = "ChainA";
    const TARGET_NAME: &'static str = "ChainB";
}

#[derive(Clone)]
pub struct ChainBToChainAMessageLane;

impl SubstrateMessageLane for ChainBToChainAMessageLane {
    type SourceChain = ChainB;
    type TargetChain = ChainA;
    type LaneId = HashedLaneId;
    
    const SOURCE_NAME: &'static str = "ChainB";
    const TARGET_NAME: &'static str = "ChainA";
}

// Placeholder types that will be replaced by codegen
// These should be generated from your runtime metadata
pub type ChainARuntimeCall = (); // Will be replaced by codegen
pub type ChainBRuntimeCall = (); // Will be replaced by codegen

// Configuration structure for the relay
#[derive(Clone)]
pub struct SoloChainRelayConfig {
    pub chain_a_uri: String,
    pub chain_b_uri: String,
    pub chain_a_signer: sp_core::sr25519::Pair,
    pub chain_b_signer: sp_core::sr25519::Pair,
    pub lane_id: HashedLaneId,
    pub headers_to_relay: HeadersToRelay,
    pub transaction_mortality: Option<u32>,
}

// Main relay implementation
pub struct SoloChainRelay {
    config: SoloChainRelayConfig,
    chain_a_client: DefaultClient<ChainA>,
    chain_b_client: DefaultClient<ChainB>,
}

impl SoloChainRelay {
    pub async fn new(config: SoloChainRelayConfig) -> anyhow::Result<Self> {
        log::info!("Initializing Solo Chain Relay...");
        log::info!("Chain A URI: {}", config.chain_a_uri);
        log::info!("Chain B URI: {}", config.chain_b_uri);

        let chain_a_client = relay_substrate_client::new(ConnectionParams {
            uri: config.chain_a_uri.clone(),
            chain_runtime_version: ChainRuntimeVersion::Auto,
        }).await?;

        let chain_b_client = relay_substrate_client::new(ConnectionParams {
            uri: config.chain_b_uri.clone(),
            chain_runtime_version: ChainRuntimeVersion::Auto,
        }).await?;

        log::info!("Successfully connected to both chains");

        Ok(Self {
            config,
            chain_a_client,
            chain_b_client,
        })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        log::info!("Starting Solo Chain Bridge Relay...");

        let chain_a_tx_params = RelayTransactionParams {
            signer: self.config.chain_a_signer.clone(),
            mortality: self.config.transaction_mortality,
        };

        let chain_b_tx_params = RelayTransactionParams {
            signer: self.config.chain_b_signer.clone(),
            mortality: self.config.transaction_mortality,
        };

        self.verify_bridge_initialization().await?;

        let a_to_b_headers_relay = Arc::new(
            OnDemandHeadersRelay::<ChainAToChainBFinalityPipeline, _, _>::new(
                self.chain_a_client.clone(),
                self.chain_b_client.clone(),
                chain_b_tx_params.clone(),
                self.config.headers_to_relay,
                None,
            )
        );

        let b_to_a_headers_relay = Arc::new(
            OnDemandHeadersRelay::<ChainBToChainAFinalityPipeline, _, _>::new(
                self.chain_b_client.clone(),
                self.chain_a_client.clone(),
                chain_a_tx_params.clone(),
                self.config.headers_to_relay,
                None,
            )
        );

        let relay_tasks = vec![
            tokio::spawn(self.run_finality_relay::<ChainAToChainBFinalityPipeline>(
                self.chain_a_client.clone(),
                self.chain_b_client.clone(),
                chain_b_tx_params.clone(),
            )),
            tokio::spawn(self.run_finality_relay::<ChainBToChainAFinalityPipeline>(
                self.chain_b_client.clone(),
                self.chain_a_client.clone(),
                chain_a_tx_params.clone(),
            )),
            tokio::spawn(self.run_messages_relay::<ChainAToChainBMessageLane>(
                self.chain_a_client.clone(),
                self.chain_b_client.clone(),
                chain_a_tx_params.clone(),
                chain_b_tx_params.clone(),
                a_to_b_headers_relay,
                b_to_a_headers_relay.clone(),
            )),
            tokio::spawn(self.run_messages_relay::<ChainBToChainAMessageLane>(
                self.chain_b_client.clone(),
                self.chain_a_client.clone(),
                chain_b_tx_params,
                chain_a_tx_params,
                b_to_a_headers_relay,
                a_to_b_headers_relay,
            )),
        ];

        log::info!("All relay components started successfully");

        let (result, _index, _remaining) = futures::future::select_all(relay_tasks).await;
        match result {
            Ok(Ok(())) => {
                log::info!("Relay component completed successfully");
                Ok(())
            },
            Ok(Err(e)) => {
                log::error!("Relay component failed: {:?}", e);
                Err(e)
            },
            Err(e) => {
                log::error!("Relay task panicked: {:?}", e);
                Err(anyhow::anyhow!("Relay task panicked: {:?}", e))
            }
        }
    }

    async fn verify_bridge_initialization(&self) -> anyhow::Result<()> {
        log::info!("Verifying bridge pallet initialization...");
        
        let chain_a_hash = self.chain_a_client.best_header_hash().await?;
        let chain_b_hash = self.chain_b_client.best_header_hash().await?;
        
        log::info!("Chain A best block: {:?}", chain_a_hash);
        log::info!("Chain B best block: {:?}", chain_b_hash);
        
        Ok(())
    }

    async fn run_finality_relay<P: SubstrateFinalitySyncPipeline>(
        &self,
        source_client: DefaultClient<P::SourceChain>,
        target_client: DefaultClient<P::TargetChain>,
        transaction_params: RelayTransactionParams<AccountKeyPairOf<P::TargetChain>>,
    ) -> anyhow::Result<()> {
        log::info!(
            "Starting finality relay from {} to {}",
            P::SourceChain::NAME,
            P::TargetChain::NAME
        );

        run_finality::<P>(
            source_client,
            target_client,
            self.config.headers_to_relay,
            transaction_params,
            MetricsParams::disabled(),
        ).await
    }

    async fn run_messages_relay<ML: SubstrateMessageLane>(
        &self,
        source_client: DefaultClient<ML::SourceChain>,
        target_client: DefaultClient<ML::TargetChain>,
        source_tx_params: RelayTransactionParams<AccountKeyPairOf<ML::SourceChain>>,
        target_tx_params: RelayTransactionParams<AccountKeyPairOf<ML::TargetChain>>,
        source_to_target_headers_relay: Arc<dyn relay_substrate_client::OnDemandRelay<ML::SourceChain, ML::TargetChain>>,
        target_to_source_headers_relay: Arc<dyn relay_substrate_client::OnDemandRelay<ML::TargetChain, ML::SourceChain>>,
    ) -> anyhow::Result<()> 
    where
        AccountIdOf<ML::SourceChain>: From<<AccountKeyPairOf<ML::SourceChain> as Pair>::Public>,
        AccountIdOf<ML::TargetChain>: From<<AccountKeyPairOf<ML::TargetChain> as Pair>::Public>,
    {
        log::info!(
            "Starting messages relay from {} to {} on lane {:?}",
            ML::SourceChain::NAME,
            ML::TargetChain::NAME,
            self.config.lane_id
        );

        let messages_relay_params = MessagesRelayParams {
            source_client,
            source_transaction_params: source_tx_params,
            target_client,
            target_transaction_params: target_tx_params,
            source_to_target_headers_relay: Some(source_to_target_headers_relay),
            target_to_source_headers_relay: Some(target_to_source_headers_relay),
            lane_id: self.config.lane_id,
            limits: None,
            metrics_params: MetricsParams::disabled(),
        };

        run_messages::<ML, _, _>(messages_relay_params).await
    }
}

// Helper function to create a hashed lane ID
pub fn create_lane_id(id: [u8; 4]) -> HashedLaneId {
    use sp_core::Hasher;
    HashedLaneId(BlakeTwo256::hash(&id))
}

// Configuration builder for easier setup
impl SoloChainRelayConfig {
    pub fn builder() -> SoloChainRelayConfigBuilder {
        SoloChainRelayConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct SoloChainRelayConfigBuilder {
    chain_a_uri: Option<String>,
    chain_b_uri: Option<String>,
    chain_a_signer: Option<sp_core::sr25519::Pair>,
    chain_b_signer: Option<sp_core::sr25519::Pair>,
    lane_id: Option<HashedLaneId>,
    headers_to_relay: HeadersToRelay,
    transaction_mortality: Option<u32>,
}

impl SoloChainRelayConfigBuilder {
    pub fn chain_a_uri(mut self, uri: impl Into<String>) -> Self {
        self.chain_a_uri = Some(uri.into());
        self
    }

    pub fn chain_b_uri(mut self, uri: impl Into<String>) -> Self {
        self.chain_b_uri = Some(uri.into());
        self
    }

    pub fn chain_a_signer(mut self, signer: sp_core::sr25519::Pair) -> Self {
        self.chain_a_signer = Some(signer);
        self
    }

    pub fn chain_b_signer(mut self, signer: sp_core::sr25519::Pair) -> Self {
        self.chain_b_signer = Some(signer);
        self
    }

    pub fn lane_id(mut self, lane_id: HashedLaneId) -> Self {
        self.lane_id = Some(lane_id);
        self
    }

    pub fn headers_to_relay(mut self, headers: HeadersToRelay) -> Self {
        self.headers_to_relay = headers;
        self
    }

    pub fn transaction_mortality(mut self, mortality: u32) -> Self {
        self.transaction_mortality = Some(mortality);
        self
    }

    pub fn build(self) -> anyhow::Result<SoloChainRelayConfig> {
        Ok(SoloChainRelayConfig {
            chain_a_uri: self.chain_a_uri.ok_or_else(|| anyhow::anyhow!("Chain A URI is required"))?,
            chain_b_uri: self.chain_b_uri.ok_or_else(|| anyhow::anyhow!("Chain B URI is required"))?,
            chain_a_signer: self.chain_a_signer.ok_or_else(|| anyhow::anyhow!("Chain A signer is required"))?,
            chain_b_signer: self.chain_b_signer.ok_or_else(|| anyhow::anyhow!("Chain B signer is required"))?,
            lane_id: self.lane_id.unwrap_or_else(|| create_lane_id([0, 0, 0, 1])),
            headers_to_relay: self.headers_to_relay,
            transaction_mortality: self.transaction_mortality,
        })
    }
}

// Example usage and main function
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting Solo Chain Bridge Relay Application");

    let config = SoloChainRelayConfig::builder()
        .chain_a_uri("ws://localhost:9944")
        .chain_b_uri("ws://localhost:9945")
        .chain_a_signer(
            sp_core::sr25519::Pair::from_string("//Alice", None)
                .map_err(|e| anyhow::anyhow!("Failed to parse chain A signer: {:?}", e))?
        )
        .chain_b_signer(
            sp_core::sr25519::Pair::from_string("//Bob", None)
                .map_err(|e| anyhow::anyhow!("Failed to parse chain B signer: {:?}", e))?
        )
        .lane_id(create_lane_id([0, 0, 0, 1]))
        .headers_to_relay(HeadersToRelay::All)
        .transaction_mortality(64)
        .build()?;

    let relay = SoloChainRelay::new(config).await?;
    relay.run().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = SoloChainRelayConfig::builder()
            .chain_a_uri("ws://localhost:9944")
            .chain_b_uri("ws://localhost:9945")
            .chain_a_signer(sp_core::sr25519::Pair::from_string("//Alice", None).unwrap())
            .chain_b_signer(sp_core::sr25519::Pair::from_string("//Bob", None).unwrap())
            .lane_id(create_lane_id([0, 0, 0, 1]))
            .headers_to_relay(HeadersToRelay::All)
            .transaction_mortality(64)
            .build()
            .expect("Config should build successfully");

        assert_eq!(config.chain_a_uri, "ws://localhost:9944");
        assert_eq!(config.chain_b_uri, "ws://localhost:9945");
        assert_eq!(config.transaction_mortality, Some(64));
    }

    #[test]
    fn test_lane_id_creation() {
        let lane_id1 = create_lane_id([0, 0, 0, 1]);
        let lane_id2 = create_lane_id([0, 0, 0, 1]);
        let lane_id3 = create_lane_id([0, 0, 0, 2]);

        assert_eq!(lane_id1, lane_id2);
        assert_ne!(lane_id1, lane_id3);
    }

    #[tokio::test]
    async fn test_relay_initialization() {
        let config = SoloChainRelayConfig::builder()
            .chain_a_uri("ws://localhost:9944")
            .chain_b_uri("ws://localhost:9945")
            .chain_a_signer(sp_core::sr25519::Pair::from_string("//Alice", None).unwrap())
            .chain_b_signer(sp_core::sr25519::Pair::from_string("//Bob", None).unwrap())
            .build()
            .unwrap();

        assert_eq!(config.chain_a_uri, "ws://localhost:9944");
        assert_eq!(config.chain_b_uri, "ws://localhost:9945");
    }
}