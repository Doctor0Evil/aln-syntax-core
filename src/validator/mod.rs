//! ALN Schema Validator - Validates shards against canonical schemas
//!
//! This module provides validation logic for all ALN schema types,
//! ensuring compliance with governance invariants and constraints.

use crate::error::AlnError;
use crate::schemas::ndm::NdmSnapshot;
use crate::schemas::sourze::SourzeManifest;
use crate::schemas::row::RowShard;
use crate::hex_stamp::verify_hex_stamp;
use serde::Serialize;

/// Schema validator for all ALN types
pub struct SchemaValidator {
    strict_mode: bool,
    offline_mode: bool,
}

impl SchemaValidator {
    /// Create a new schema validator
    pub fn new() -> Self {
        Self {
            strict_mode: true,
            offline_mode: true,
        }
    }

    /// Enable/disable strict mode
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    /// Enable/disable offline mode
    pub fn with_offline_mode(mut self, offline: bool) -> Self {
        self.offline_mode = offline;
        self
    }

    /// Validate NDM snapshot against schema invariants
    pub fn validate_ndm(&self, snapshot: &NdmSnapshot) -> Result<(), AlnError> {
        // Validate K-score range
        if snapshot.k_score < 0.0 || snapshot.k_score > 1.0 {
            return Err(AlnError::ValidationFailed(
                "K-score must be between 0.0 and 1.0".to_string(),
            ));
        }

        // Validate state transition monotonicity
        if snapshot.current_state < snapshot.previous_state {
            return Err(AlnError::ValidationFailed(
                "NDM state transitions must be monotone (no upgrades)".to_string(),
            ));
        }

        // Validate hex-stamp
        if !verify_hex_stamp(snapshot, &snapshot.hex_stamp) {
            return Err(AlnError::ValidationFailed(
                "Hex-stamp verification failed".to_string(),
            ));
        }

        // Validate ROW reference
        if snapshot.row_reference.is_empty() {
            return Err(AlnError::ValidationFailed(
                "ROW reference required for NDM snapshot".to_string(),
            ));
        }

        // Validate Cyberspectre trace ID
        if snapshot.cyberspectre_trace_id.is_empty() {
            return Err(AlnError::ValidationFailed(
                "Cyberspectre trace ID required for audit".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate Sourze manifest against capability lattice
    pub fn validate_sourze(&self, manifest: &SourzeManifest) -> Result<(), AlnError> {
        // Validate multi-DID envelope
        if manifest.did_owner.is_empty()
            || manifest.did_host.is_empty()
            || manifest.did_auditor.is_empty()
        {
            return Err(AlnError::ValidationFailed(
                "Multi-DID envelope required (owner, host, auditor)".to_string(),
            ));
        }

        // Validate non-weaponization for NanoswarmCtrl
        if manifest.capabilities.contains(&crate::schemas::sourze::SourzeCapability::NanoswarmCtrl)
        {
            if manifest.non_weapon_envelope.is_none() {
                return Err(AlnError::ValidationFailed(
                    "NanoswarmCtrl requires non-weapon envelope".to_string(),
                ));
            }
        }

        // Validate forbidden capability combinations
        self.validate_forbidden_combos(&manifest.capabilities)?;

        // Validate EcoVector floor
        if manifest.eco_vector.eco_impact_score < manifest.eco_vector.eco_floor_minimum {
            return Err(AlnError::ValidationFailed(
                "EcoVector below minimum floor".to_string(),
            ));
        }

        // Validate hex-stamp
        if !verify_hex_stamp(manifest, &manifest.hex_stamp) {
            return Err(AlnError::ValidationFailed(
                "Hex-stamp verification failed".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate ROW shard
    pub fn validate_row(&self, shard: &RowShard) -> Result<(), AlnError> {
        // Validate hex-stamp format
        if !shard.hex_stamp.starts_with("0x") || shard.hex_stamp.len() != 66 {
            return Err(AlnError::ValidationFailed(
                "Invalid hex-stamp format".to_string(),
            ));
        }

        // Validate Cyberspectre trace ID
        if shard.cyberspectre_trace_id.is_empty() {
            return Err(AlnError::ValidationFailed(
                "Cyberspectre trace ID required".to_string(),
            ));
        }

        // Validate ledger anchor
        if shard.ledger_anchor.transaction_id.is_empty() {
            return Err(AlnError::ValidationFailed(
                "Ledger anchor required".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate forbidden capability combinations
    fn validate_forbidden_combos(
        &self,
        capabilities: &[crate::schemas::sourze::SourzeCapability],
    ) -> Result<(), AlnError> {
        use crate::schemas::sourze::SourzeCapability::*;

        let has_ctrl = capabilities.contains(&NanoswarmCtrl);
        let has_net_server = capabilities.contains(&NetServer);
        let has_fs_write = capabilities.contains(&FsWrite);
        let has_usb = capabilities.contains(&UsbHid);
        let has_serial = capabilities.contains(&SerialMcu);
        let has_gpu = capabilities.contains(&GpuCompute);

        if has_ctrl && has_net_server {
            return Err(AlnError::ValidationFailed(
                "Forbidden combo: NANOSWARM_CTRL + NETSERVER".to_string(),
            ));
        }

        if has_ctrl && has_fs_write && (has_usb || has_serial) {
            return Err(AlnError::ValidationFailed(
                "Forbidden combo: NANOSWARM_CTRL + FSWRITE + (USB_HID|SERIAL_MCU)".to_string(),
            ));
        }

        if has_ctrl && has_gpu {
            return Err(AlnError::ValidationFailed(
                "Forbidden combo: NANOSWARM_CTRL + GPU_COMPUTE".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for SchemaValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = SchemaValidator::new();
        assert!(validator.strict_mode);
        assert!(validator.offline_mode);
    }
}
