# Horizon Stream: Decentralized Capability-based Access Control System

## Overview

This document outlines a decentralized capability-based access control system for the Horizon Stream platform. The system enables direct interaction between content creators and storage providers without requiring a centralized authority or tokenomics.

## Core Components

### 1. Capability Certificates

Capability certificates are the fundamental unit of authorization in the system. They are cryptographically signed objects that grant specific rights to perform actions on content.

```rust
struct Capability {
    // Unique identifier for this capability
    id: UUID,
    // Content identifier in Iroh
    content_cid: String,
    // Creator's public key
    creator_key: PublicKey,
    // Type of capability
    capability_type: CapabilityType,
    // Optional restrictions
    constraints: CapabilityConstraints,
    // Cryptographic proof chain
    proof: CapabilityProof,
    // Validity period
    validity: TimeRange,
}

enum CapabilityType {
    Store,      // Right to store and replicate content
    Serve,      // Right to serve content to users
    View,       // Right to access content
    Share,      // Right to create attenuated capabilities
    Manage,     // Right to modify content metadata
}

struct CapabilityConstraints {
    // Geographic restrictions
    geo_restrictions: Option<Vec<Region>>,
    // Maximum bandwidth
    bandwidth_limit: Option<u64>,
    // Quality levels allowed
    quality_levels: Vec<QualityLevel>,
    // Custom constraints as key-value pairs
    custom: HashMap<String, String>,
}
```

### 2. Capability Proofs

Proofs maintain the chain of delegation and ensure capabilities cannot be forged or expanded.

```rust
struct CapabilityProof {
    // Chain of delegations from root capability
    delegation_chain: Vec<Delegation>,
    // Cryptographic signatures for each delegation
    signatures: Vec<Signature>,
}

struct Delegation {
    // Who delegated the capability
    delegator: PublicKey,
    // Who received the capability
    delegate: PublicKey,
    // Timestamp of delegation
    timestamp: DateTime<Utc>,
    // Additional restrictions added
    restrictions: Vec<Constraint>,
}
```

## Core Operations

### 1. Content Publishing

When a creator publishes content, they establish the root capabilities:

```rust
async fn publish_content(
    content: Content,
    metadata: Metadata,
    initial_capabilities: Vec<Capability>,
) -> Result<ContentRecord, Error> {
    // 1. Generate content hash
    let content_hash = hash_content(&content);
    
    // 2. Create root capabilities
    let root_capabilities = generate_root_capabilities(
        content_hash,
        &metadata,
        initial_capabilities,
    );
    
    // 3. Sign everything with creator's key
    let signed_package = sign_content_package(
        content,
        metadata,
        root_capabilities,
    );
    
    // 4. Store in Iroh
    let content_record = store_in_iroh(signed_package).await?;
    
    Ok(content_record)
}
```

### 2. Storage Provider Participation

Storage providers validate and honor capabilities:

```rust
async fn handle_storage_request(
    content: Content,
    capability: Capability,
) -> Result<StorageReceipt, Error> {
    // 1. Validate capability chain
    validate_capability_chain(&capability)?;
    
    // 2. Check storage rights
    if !capability.has_permission(CapabilityType::Store) {
        return Err(Error::InsufficientCapability);
    }
    
    // 3. Verify constraints
    verify_storage_constraints(&capability.constraints)?;
    
    // 4. Store content
    let storage_receipt = store_content(content).await?;
    
    // 5. Register capability
    register_capability(capability, storage_receipt.location)?;
    
    Ok(storage_receipt)
}
```

### 3. Content Access

Users present capabilities to access content:

```rust
async fn access_content(
    content_id: &str,
    capability: &Capability,
    proof: &CapabilityProof,
) -> Result<ContentStream, Error> {
    // 1. Verify capability chain
    verify_capability_chain(capability, proof)?;
    
    // 2. Check current time bounds
    verify_time_validity(capability)?;
    
    // 3. Validate constraints
    validate_access_constraints(&capability.constraints)?;
    
    // 4. Setup content stream with enforced constraints
    let stream = setup_constrained_stream(
        content_id,
        &capability.constraints,
    ).await?;
    
    Ok(stream)
}
```

## Capability Delegation

### 1. Attenuating Capabilities

Capabilities can be delegated with additional restrictions:

```rust
impl Capability {
    fn attenuate(
        &self,
        new_constraints: CapabilityConstraints,
        delegate: PublicKey,
    ) -> Result<Capability, Error> {
        // 1. Verify we have delegation rights
        if !self.can_delegate() {
            return Err(Error::NoDelegationRight);
        }
        
        // 2. Combine constraints (only more restrictive)
        let combined_constraints = combine_constraints(
            &self.constraints,
            &new_constraints,
        )?;
        
        // 3. Create new capability
        let new_capability = Capability {
            id: generate_uuid(),
            content_cid: self.content_cid.clone(),
            creator_key: self.creator_key.clone(),
            capability_type: self.capability_type,
            constraints: combined_constraints,
            proof: extend_proof_chain(self.proof.clone(), delegate),
            validity: self.validity,
        };
        
        // 4. Sign the new capability
        sign_capability(&new_capability)
    }
}
```

### 2. Validation

Ensuring capability chains are valid:

```rust
fn validate_capability_chain(
    capability: &Capability,
    proof: &CapabilityProof,
) -> Result<bool, Error> {
    // 1. Verify the root capability
    verify_root_capability(&capability)?;
    
    // 2. Validate each delegation in the chain
    for delegation in &proof.delegation_chain {
        verify_delegation(delegation)?;
        verify_signature(&delegation.signature)?;
        verify_constraints(&delegation.restrictions)?;
    }
    
    // 3. Ensure no capability expansion
    verify_no_expansion(capability, proof)?;
    
    Ok(true)
}
```

## Advanced Features

### 1. Anonymous Access

Support for privacy-preserving access:

```rust
struct BlindCapability {
    // Blinded content identifier
    blinded_cid: BlindedValue,
    // Anonymous credentials
    credentials: AnonymousCredentials,
    // Zero-knowledge proof of capability
    zkp: ZKProof,
}
```

### 2. Rate Limiting

Built-in support for usage quotas:

```rust
struct RateLimit {
    // Time window for quota
    window: Duration,
    // Maximum requests in window
    max_requests: u32,
    // Maximum bandwidth in window
    max_bandwidth: u64,
    // Current usage metrics
    usage: UsageMetrics,
}
```

## Security Considerations

1. **Capability Revocation**
   - Use short-lived capabilities
   - Implement revocation checking
   - Support emergency revocation

2. **Replay Protection**
   - Include nonces in capabilities
   - Implement timestamp checking
   - Track used capabilities

3. **Privacy Protection**
   - Minimize metadata exposure
   - Support anonymous credentials
   - Implement unlinkable access

## Implementation Guidelines

### 1. Storage Provider Implementation

```rust
trait StorageProvider {
    // Accept new content with capability
    async fn store(&self, content: Content, cap: Capability) -> Result<(), Error>;
    
    // Serve content to users with capability
    async fn serve(&self, request: Request, cap: Capability) -> Result<Stream, Error>;
    
    // Verify capabilities
    fn verify_capability(&self, cap: &Capability) -> Result<bool, Error>;
}
```

### 2. Content Creator Implementation

```rust
trait ContentCreator {
    // Publish new content
    async fn publish(&self, content: Content) -> Result<ContentRecord, Error>;
    
    // Grant new capabilities
    fn grant_capability(&self, params: CapabilityParams) -> Result<Capability, Error>;
    
    // Revoke capabilities
    fn revoke_capability(&self, cap_id: UUID) -> Result<(), Error>;
}
```

### 3. User Client Implementation

```rust
trait Client {
    // Request content access
    async fn access_content(&self, cap: Capability) -> Result<Stream, Error>;
    
    // Manage received capabilities
    fn store_capability(&self, cap: Capability) -> Result<(), Error>;
    
    // Share capabilities
    fn share_capability(&self, cap: Capability, with: PublicKey) -> Result<Capability, Error>;
}
```

## Future Considerations

1. **Scalability Improvements**
   - Capability aggregation
   - Hierarchical capabilities
   - Caching strategies

2. **Additional Features**
   - Multi-party capabilities
   - Conditional capabilities
   - Capability templates

3. **Integration Points**
   - CDN integration
   - Analytics systems
   - Content management systems

## Conclusion

This capability-based system provides a robust foundation for decentralized access control in the Horizon Stream platform. It enables secure content sharing and delegation without requiring centralized authorities or blockchain technologies.
