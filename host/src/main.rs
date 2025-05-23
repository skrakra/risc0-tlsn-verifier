use std::{fs, path::PathBuf};

use anyhow::Result;
use methods::{PROOF_VERIFIER_GUEST_ELF, PROOF_VERIFIER_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct VerificationOutput {
    is_valid: bool,
    server_name: String,
    score: Option<u64>,
    error: Option<String>,
}

fn main() -> Result<()> {
    // Read proof.json
    let proof_path = PathBuf::from("data").join("proof.json");
    let proof_json = fs::read_to_string(&proof_path).map_err(|e| {
        anyhow::anyhow!("Failed to read proof file {}: {}", proof_path.display(), e)
    })?;

    let env = ExecutorEnv::builder().write(&proof_json)?.build()?;

    let prover = default_prover();

    println!("Running the prover...");
    // Execute guest, produce Receipt
    let prove_info = prover.prove(env, PROOF_VERIFIER_GUEST_ELF)?;
    println!("Proving finished.");

    let receipt = prove_info.receipt;

    // Verify receipt against expected ImageID
    receipt.verify(PROOF_VERIFIER_GUEST_ID)?;
    println!("Receipt verification successful!");

    // Decode journal
    let output: VerificationOutput = receipt.journal.decode()?;

    println!("\nGuest Output:");
    println!("  Is Valid:     {}", output.is_valid);
    println!("  Server Name:  {}", output.server_name);
    if let Some(score) = output.score {
        println!("  Score:        {}", score);
        if output.is_valid && score > 5 {
            println!("\nSuccessfully verified proof and extracted score: {} (above threshold of 5)", score);
        }
    }
    if let Some(err_msg) = output.error {
        println!("  Error:        {}", err_msg);
    }

    Ok(())
}