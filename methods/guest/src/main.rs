#![no_std]
#![no_main]

extern crate alloc;
use alloc::{
    format, str,
    string::{String, ToString},
};

use bincode;
use hex;
use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use tlsn_core::{
    presentation::{Presentation, PresentationOutput},
    signing::VerifyingKey as TlsnVerifyingKey,
    CryptoProvider,
};

risc0_zkvm::guest::entry!(main);

/// 33-byte compressed SEC-1 form of the Notary's public key
const EXPECTED_COMPRESSED_HEX: &str =
    "02d4cbba990b0c2eb1dd45b29c7d26075299f1ea39317f35140e6ef71e703beda7";

#[derive(Debug, Serialize, Deserialize)]
struct VerificationOutput {
    is_valid: bool,
    server_name: String,
    score: Option<u64>,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InputProofJson {
    #[serde(rename = "presentationJson")]
    presentation_json: InputPresentationData,
}

#[derive(Debug, Serialize, Deserialize)]
struct InputPresentationData {
    version: String,
    data: String,
}

fn main() {
    let proof_json: String = env::read();

    let mut output = VerificationOutput {
        is_valid: false,
        server_name: String::new(),
        score: None,
        error: None,
    };

    // Parse outer JSON
    let input: InputProofJson = match serde_json::from_str(&proof_json) {
        Ok(v) => v,
        Err(e) => {
            output.error = Some(format!("Failed to parse outer JSON: {}", e));
            env::commit(&output);
            return;
        }
    };

    // Hex-decode bincode payload
    let proof_bytes = match hex::decode(&input.presentation_json.data) {
        Ok(b) => b,
        Err(e) => {
            output.error = Some(format!("Failed to hex-decode data: {}", e));
            env::commit(&output);
            return;
        }
    };

    // Bincode-deserialize into Presentation
    let tlsn_presentation: Presentation = match bincode::deserialize(&proof_bytes) {
        Ok(p) => p,
        Err(e) => {
            output.error = Some(format!("Bincode deserialize failed: {}", e));
            env::commit(&output);
            return;
        }
    };

    // Key check: compare compressed form directly
    let embedded_vk: &TlsnVerifyingKey = tlsn_presentation.verifying_key();
    let embedded_hex = hex::encode(&embedded_vk.data);
    if embedded_hex != EXPECTED_COMPRESSED_HEX {
        output.error = Some(format!(
            "Key mismatch:\n  embedded = {}\n  expected = {}",
            embedded_hex, EXPECTED_COMPRESSED_HEX,
        ));
        env::commit(&output);
        return;
    }

    // All checks passed: verify Presentation
    let provider = CryptoProvider::default();
    let pres_out: PresentationOutput = match tlsn_presentation.verify(&provider) {
        Ok(o) => o,
        Err(e) => {
            output.error = Some(format!("Presentation.verify() failed: {:?}", e));
            env::commit(&output);
            return;
        }
    };

    // Extract server_name
    if let Some(sn) = pres_out.server_name {
        output.server_name = sn.to_string();
    }
    output.is_valid = true;
    // Extract score if present
    if let Some(transcript) = pres_out.transcript {
        if let Ok(s) = str::from_utf8(transcript.received_unsafe()) {
            if let Some(val) = s.split("score=").nth(1) {
                output.score = val
                    .split(&['&', '"'][..])
                    .next()
                    .and_then(|num| num.parse().ok());
            }
        }
    }

    // Examplary: enforce minimum score threshold 
    match output.score {
        Some(score_val) if score_val > 5 => {
            // OK: above threshold
        }
        Some(score_val) => {
            output.error = Some(format!("Score {} is below the required threshold of 5", score_val));
            output.is_valid = false;
            env::commit(&output);
            return;
        }
        None => {
            output.error = Some("Score missing or could not be parsed".to_string());
            output.is_valid = false;
            env::commit(&output);
            return;
        }
    }

    env::commit(&output);
}