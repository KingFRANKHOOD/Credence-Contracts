# Emergency Pause Mechanism

## Overview

All Credence contracts include a comprehensive emergency pause mechanism that allows authorized parties to temporarily halt all state-changing operations while preserving read access. This mechanism provides a critical safety layer for emergency situations.

## Architecture

### Core Components

1. **Pause State**: A boolean flag stored in contract storage indicating whether the contract is paused
2. **Pause Signers**: Authorized addresses that can propose pause/unpause actions
3. **Pause Threshold**: Minimum number of approvals required to execute pause/unpause proposals
4. **Proposal System**: Multi-signature workflow for pause/unpause decisions

### Contracts with Pause Mechanism

- `credence_registry` - Identity registration management
- `credence_arbitration` - Dispute resolution system  
- `credence_delegation` - Attestation delegation management
- `credence_treasury` - Fee collection and withdrawal management
- `credence_bond` - Identity bond creation and management
- `admin` - Admin role management system

## Pause Mechanism API

### Core Functions

#### `is_paused() -> bool`
Returns the current pause state of the contract.

#### `pause(caller: Address) -> Option<u64>`
Proposes to pause the contract. Returns proposal ID if multi-sig is required, None if threshold is 0.

#### `unpause(caller: Address) -> Option<u64>`
Proposes to unpause the contract. Returns proposal ID if multi-sig is required, None if threshold is 0.

### Multi-signature Management

#### `set_pause_signer(admin: Address, signer: Address, enabled: bool)`
Add or remove pause signers. Only contract admins can manage signers.

#### `set_pause_threshold(admin: Address, threshold: u32)`
Set the minimum number of approvals required. Threshold cannot exceed signer count.

#### `approve_pause_proposal(signer: Address, proposal_id: u64)`
Approve a pause/unpause proposal. Only authorized pause signers can approve.

#### `execute_pause_proposal(proposal_id: u64)`
Execute a pause/unpause proposal once sufficient approvals are collected.

## Operational Modes

### Mode 1: Admin-only (Threshold = 0)
- Single admin can pause/unpause immediately
- No proposal system required
- Fastest response time for emergencies

### Mode 2: Multi-signature (Threshold > 0)
- Requires multiple approvals for pause/unpause
- Proposal-based workflow
- Higher security through distributed control

## Security Considerations

### Pause State Behavior
- **When Paused**: All state-changing functions are blocked with `contract is paused` error
- **When Paused**: Read-only functions continue to work normally
- **When Paused**: Pause management functions remain available for recovery

### Authorization Model
- **Admin Functions**: Require SuperAdmin role (in admin contract) or Admin address (in other contracts)
- **Pause Signers**: Can be any addresses set by contract admins
- **Self-protection**: Pause mechanism cannot be used to block itself

### Event Emission
All pause operations emit events for audit trails:
- `paused(proposal_id)` - Contract paused
- `unpaused(proposal_id)` - Contract unpaused  
- `pause_proposed(proposal_id, action)` - New proposal created
- `pause_approved(proposal_id, signer)` - Proposal approved
- `pause_signer_set(signer, enabled)` - Signer status changed
- `pause_threshold_set(threshold)` - Threshold updated

## Emergency Response Procedure

### Immediate Response (Admin-only Mode)
```rust
// Admin immediately pauses the contract
contract.pause(admin_address);
```

### Coordinated Response (Multi-sig Mode)
```rust
// Multiple signers approve pause proposal
let proposal_id = contract.pause(signer1_address);
contract.approve_pause_proposal(signer2_address, proposal_id);
contract.approve_pause_proposal(signer3_address, proposal_id);
contract.execute_pause_proposal(proposal_id);
```

### Recovery Process
```rust
// Once emergency is resolved
let proposal_id = contract.unpause(signer1_address);
contract.approve_pause_proposal(signer2_address, proposal_id);
contract.execute_pause_proposal(proposal_id);
```

## Configuration Recommendations

### Production Environment
- Set threshold to majority of signers (e.g., 3 of 5)
- Use geographically distributed signers
- Regularly test pause/unpause procedures
- Monitor pause events in real-time

### Development Environment
- Set threshold to 0 for rapid testing
- Use single admin account
- Test both paused and unpaused states

## Testing Coverage

All pause mechanism implementations include comprehensive tests covering:
- Basic pause/unpause functionality
- Multi-signature proposal workflow
- Threshold enforcement
- Read-only operation preservation
- State-changing operation blocking
- Error conditions and edge cases

## Integration Notes

### Contract Integration Pattern
Each contract follows the same integration pattern:
1. Add pause-related `DataKey` entries to storage enum
2. Initialize pause state in contract `initialize()` function
3. Add `pausable::require_not_paused(&e)` to all state-changing functions
4. Expose pause management entrypoints
5. Include comprehensive test coverage

### Upgrade Compatibility
The pause mechanism is designed to be:
- Backward compatible with existing contracts
- Non-disruptive to current functionality
- Easily enabled/disabled through configuration
- Upgrade-safe through storage versioning

## Monitoring and Alerting

### Key Metrics to Monitor
- Pause state changes
- Proposal creation and approval rates
- Threshold configuration changes
- Signer status modifications

### Alert Conditions
- Contract enters paused state
- High rate of pause proposals
- Failed pause execution attempts
- Unauthorized pause access attempts

## Troubleshooting

### Common Issues

**Contract won't pause**
- Verify caller is authorized admin or pause signer
- Check if threshold is met for multi-sig mode
- Ensure contract is not already paused

**Contract won't unpause**  
- Verify sufficient approvals for proposal
- Check proposal ID is valid
- Ensure proposal has not already been executed

**Read operations failing**
- Read operations should always work when paused
- Check if function incorrectly includes pause check
- Verify error is not from other validation logic

### Emergency Recovery
If pause mechanism becomes inaccessible:
1. Contract upgrade can reset pause state
2. Admin transfer can restore access
3. Multi-sig threshold can be reduced to 0
4. Last resort: contract migration to new instance

## Future Enhancements

Planned improvements to the pause mechanism:
- Time-based automatic unpause
- Granular pause levels (partial functionality)
- Emergency override keys
- Cross-contract coordinated pausing
- Integration with external monitoring systems
