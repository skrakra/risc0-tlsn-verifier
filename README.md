# risc0-tlsn-verifier

Verify TLSNotary proof inside RISC0 zkVM. Example use case here: parse a score and verify that it exceeds a specific threshold.

## Prerequisites

You need to install Risc0- and LLVM-toolchain

## Setup

1. **Adjust linker script path**

   Open `.cargo/config.toml` and modify the `linker` path to the absolute path on your machine:
   ```bash
    linker="/Users/.../risc0-tlsn-verifier/riscv32im-linker.sh"
   ```

   > Note: Relative paths currently are not resolving correctly inside sub-crates,
   > so an absolute path is required until a workspace-level fix is implemented.
   
## Building

Use Docker to build your guest code with the RISC0 toolchain:

```bash
RISC0_USE_DOCKER=1 cargo build --workspace --release
```

This will compile both the guest (zkVM) and host binaries under `target/release`.

## Running in Development Mode

To skip the extensive proof generation run quickly in dev mode:

```bash
RISC0_USE_DOCKER=1 \
RISC0_DEV_MODE=1 \
cargo run -p host --release -- data/proof.json
```

- `-p host` runs the host application.
- `data/proof.json` path to TLSNotary proof to be verified.
