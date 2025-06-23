Chains don't automatically discover each other. You need to manually configure the relayer with:
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│    CHAIN A      │    │     RELAYER     │    │    CHAIN B      │
│   (Runtime)     │◄──►│   (Off-chain)   │◄──►│   (Runtime)     │
│                 │    │                 │    │                 │
│ Bridge Pallets  │    │ Network Client  │    │ Bridge Pallets  │
│ - Grandpa       │    │ - RPC calls     │    │ - Grandpa       │
│ - Messages      │    │ - HTTP/WS       │    │ - Messages      │
│ - Relayers      │    │ - IP:PORT       │    │ - Relayers      │
└─────────────────┘    └─────────────────┘    └─────────────────┘

# Example relayer configuration
CHAIN_A_RPC="http://192.168.1.100:9944"  # Chain A server
CHAIN_B_RPC="http://192.168.1.200:9945"  # Chain B server


We need to create a relayer service that:

Connects to both chains via RPC
Monitors finalized blocks on both chains
Submits headers and messages between chains



📊 Communication Flow:
┌─────────────────────────────────────────────────────────────────┐
│                    BRIDGE COMMUNICATION FLOW                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ 1. Chain A produces block → finalized by GRANDPA               │
│ 2. Relayer detects via RPC: chain_getFinalizedHead             │
│ 3. Relayer fetches header + justification                      │
│ 4. Relayer submits to Chain B: bridgeGrandpa.submitHeader      │
│ 5. Chain B verifies and stores header                          │
│                                                                 │
│ 6. User sends cross-chain message on Chain A                   │
│ 7. Relayer detects outbound message                            │
│ 8. Relayer generates delivery proof                            │
│ 9. Relayer submits to Chain B: bridgeMessages.receiveMessage   │
│ 10. Chain B processes message via MessageDispatcher            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘