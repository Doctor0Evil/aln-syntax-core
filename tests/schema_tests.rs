//! ALN Schema Test Suite - Automated validation tests
//!
//! This test suite validates all schema types against invariants,
//! ensures hex-stamp integrity, and verifies governance constraints.

use aln_syntax_core::schemas::ndm::{NdmSnapshot, NdmState};
use aln_syntax_core::schemas::sourze::{SourzeManifest, SourzeCapability};
use aln_syntax_core::validator::SchemaValidator;
use aln_syntax_core::hex_stamp::generate_hex_stamp;
use uuid::Uuid;

#[test]
fn test_ndm_snapshot_validation() {
    let validator = SchemaValidator::new();
    
    let snapshot = NdmSnapshot {
        session_id: Uuid::new_v4(),
        timestamp: 1741104000,
        k_score: 0.5,
        r_score: 0.3,
        e_score: 0.2,
        current_state: NdmState::Monitoring,
        previous_state: NdmState::Normal,
        suspicion_triggers: vec!["unauthorized_did_session".to_string()],
        row_reference: "row:test:123".to_string(),
        cyberspectre_trace_id: "cyb:test:456".to_string(),
        hex_stamp: String::new(), // Will be generated
    };

    // Generate hex-stamp
    let stamp = generate_hex_stamp(&snapshot);
    
    // Validate
    assert!(validator.validate_ndm(&snapshot).is_ok());
}

#[test]
fn test_ndm_monotone_degradation() {
    let validator = SchemaValidator::new();
    
    // Valid: Normal -> Monitoring (degradation)
    let valid_transition = NdmSnapshot {
        session_id: Uuid::new_v4(),
        timestamp: 1741104000,
        k_score: 0.5,
        r_score: 0.3,
        e_score: 0.2,
        current_state: NdmState::Monitoring,
        previous_state: NdmState::Normal,
        suspicion_triggers: vec![],
        row_reference: "row:test:123".to_string(),
        cyberspectre_trace_id: "cyb:test:456".to_string(),
        hex_stamp: String::new(),
    };

    // Invalid: Monitoring -> Normal (upgrade, violates monotonicity)
    let invalid_transition = NdmSnapshot {
        session_id: Uuid::new_v4(),
        timestamp: 1741104000,
        k_score: 0.2,
        r_score: 0.3,
        e_score: 0.2,
        current_state: NdmState::Normal,
        previous_state: NdmState::Monitoring,
        suspicion_triggers: vec![],
        row_reference: "row:test:123".to_string(),
        cyberspectre_trace_id: "cyb:test:456".to_string(),
        hex_stamp: String::new(),
    };

    assert!(validator.validate_ndm(&valid_transition).is_ok());
    assert!(validator.validate_ndm(&invalid_transition).is_err());
}

#[test]
fn test_sourze_forbidden_combos() {
    let validator = SchemaValidator::new();
    
    // Valid: NanoswarmCtrl without forbidden combos
    let valid_manifest = SourzeManifest {
        manifest_id: Uuid::new_v4(),
        did_owner: "bostrom1owner".to_string(),
        did_host: "bostrom1host".to_string(),
        did_auditor: "bostrom1auditor".to_string(),
        capabilities: vec![SourzeCapability::NanoswarmCtrl, SourzeCapability::NetClient],
        eco_vector: create_test_eco_vector(),
        ndm_ceiling: 0.3,
        non_weapon_envelope: Some(create_test_non_weapon_envelope()),
        code_anchor_hash: "0xabc123".to_string(),
        zes_envelope: "zes:encrypted".to_string(),
        authorship_proof: create_test_authorship_proof(),
        timestamp: 1741104000,
        hex_stamp: String::new(),
    };

    // Invalid: NanoswarmCtrl + NetServer (forbidden)
    let invalid_manifest = SourzeManifest {
        manifest_id: Uuid::new_v4(),
        did_owner: "bostrom1owner".to_string(),
        did_host: "bostrom1host".to_string(),
        did_auditor: "bostrom1auditor".to_string(),
        capabilities: vec![SourzeCapability::NanoswarmCtrl, SourzeCapability::NetServer],
        eco_vector: create_test_eco_vector(),
        ndm_ceiling: 0.3,
        non_weapon_envelope: Some(create_test_non_weapon_envelope()),
        code_anchor_hash: "0xabc123".to_string(),
        zes_envelope: "zes:encrypted".to_string(),
        authorship_proof: create_test_authorship_proof(),
        timestamp: 1741104000,
        hex_stamp: String::new(),
    };

    assert!(validator.validate_sourze(&valid_manifest).is_ok());
    assert!(validator.validate_sourze(&invalid_manifest).is_err());
}

#[test]
fn test_hex_stamp_verification() {
    let snapshot = NdmSnapshot {
        session_id: Uuid::new_v4(),
        timestamp: 1741104000,
        k_score: 0.5,
        r_score: 0.3,
        e_score: 0.2,
        current_state: NdmState::Normal,
        previous_state: NdmState::Normal,
        suspicion_triggers: vec![],
        row_reference: "row:test:123".to_string(),
        cyberspectre_trace_id: "cyb:test:456".to_string(),
        hex_stamp: String::new(),
    };

    let stamp = generate_hex_stamp(&snapshot);
    assert!(stamp.starts_with("0x"));
    assert_eq!(stamp.len(), 66); // 0x + 64 hex chars
}

// Helper functions
fn create_test_eco_vector() -> aln_syntax_core::schemas::eco::EcoVector {
    aln_syntax_core::schemas::eco::EcoVector {
        gco2_per_joule: 0.001,
        eco_impact_score: 0.5,
        energy_autonomy_pct: 0.8,
        eco_floor_minimum: 0.3,
    }
}

fn create_test_non_weapon_envelope() -> aln_syntax_core::schemas::sourze::NanoswarmNonWeaponEnvelope {
    aln_syntax_core::schemas::sourze::NanoswarmNonWeaponEnvelope {
        envelope_id: Uuid::new_v4(),
        permitted_missions: vec!["ecological_restoration".to_string()],
        forbidden_missions: vec!["kinetic_damage".to_string()],
        effect_type: "eco".to_string(),
        mission_class: "restoration".to_string(),
        requires_multi_sig: true,
    }
}

fn create_test_authorship_proof() -> aln_syntax_core::schemas::sourze::AuthorshipProof {
    aln_syntax_core::schemas::sourze::AuthorshipProof {
        author_dids: vec!["bostrom1author".to_string()],
        row_reference: "row:authorship:123".to_string(),
        googolswarm_tx_id: "gs:tx:456".to_string(),
        git_signed_tag: "v1.0.0".to_string(),
    }
}
