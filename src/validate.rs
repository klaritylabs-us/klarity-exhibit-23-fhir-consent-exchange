
use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_consent_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    if v.get("resourceType").and_then(|x| x.as_str()) != Some("Consent") {
        r.push(ValidationIssue::error("resourceType", "expected Consent", "consent-type"));
    }
    if v.get("status").and_then(|x| x.as_str()) != Some("active") {
        r.push(ValidationIssue::error("status", "exchange Consent is usually active", "consent-active"));
    }
    if v.pointer("/patient/reference").is_none() {
        r.push(ValidationIssue::error("patient", "patient reference required", "consent-patient"));
    }
    if v.get("scope").is_none() {
        r.push(ValidationIssue::error("scope", "scope codings document what is consented", "consent-scope"));
    }
    if v.pointer("/provision/period").is_none() {
        r.push(ValidationIssue::warning("provision.period", "provision period clarifies expiry", "consent-period"));
    }
    Ok(r)
}
