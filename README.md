# risc0-tlsn-verifier

> Verify TLSNotary proof inside RISC0 zkVM. Example use case: parse a score and verify that it exceeds a specific threshold.

## Setup

**Make the linker script executable**

   From the project root, give the script execute permissions:
   ```bash
   chmod +x riscv32im-linker.sh
   ```
   and export an environment variable pointing at your local script, so Cargo/rustc (inside the R0 container) can locate and run it:
   ```bash
   export HOST_LINKER="$PWD/riscv32im-linker.sh"
   ```

## Building

Use Docker to build your guest code with the RISC0 toolchain:

```bash
RISC0_USE_DOCKER=1 \
  CARGO_TARGET_RISCV32IM_RISC0_ZKVM_ELF_LINKER="$HOST_LINKER" \
  cargo build --workspace --release
```

This will compile both the guest (zkVM) and host binaries under `target/release`.

## Running in Development Mode

To skip the extensive proof generation run quickly in dev mode:

```bash
RISC0_USE_DOCKER=1 \
RISC0_DEV_MODE=1 \
cargo run -p host --release -- data/proof.json
```
- `data/proof.json` path to TLSNotary proof to be verified.
