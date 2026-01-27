🛡️ Tetra (V1.2.0 - Rust Powered)

Core: KISS (Keep It Simple Stupid)

A high-performance, policy-driven token management and execution CLI. It features Gzip compression, XOR encryption, and intelligent color control through global settings.

🚀 Key Features

    Global Policy Control: Centrally manage output colors, compression levels, and encryption status via settings.toml.

    Hybrid Storage Engine: Automatically switches between Base64 encoded binary and Raw Text based on your security/compression settings.

    Strict Configuration Merging: Seamlessly combines global environment preferences with local project data.

    Multi-Type Execution: Out-of-the-box support for Web, Code, and Pointer token types.

🛠️ Installation
```bash
curl -sS https://tetra-install.64bit.kr | sh
```
📖 Usage
1. Configuration

    Global Settings: Define the program's behavior across the entire system.
    Bash

tetra global.init

    Path: ~/.config/tetra/settings.toml

    Parameters: use_color, use_compression, compression_level, use_encryption.

Local Project Config: Define the specific data for your tokens.
Bash

    tetra config.init

        Path: ./config.toml (in your current working directory).

2. Primary Commands

    Token Creation: Generates a token file based on local config and global policies.
    Bash

tetra token.ize

Token Inspection: Decrypts and decompresses the token content for a quick preview.
Bash

    tetra token.read <file.token>

3. Automated Executor (tkexecute)

The standalone executor that detects the token type and performs the action (HTTP request, Script execution, etc.).
Bash

tkexecute my_script.token

⚙️ Global Config Policy (settings.toml)
Field	Default	Description
use_color	true	Set to false for a completely colorless (No-ANSI) terminal output.
use_compression	true	Enables Gzip compression to minimize token size.
compression_level	6	Adjusts Gzip intensity (0: None to 9: Maximum).
use_encryption	true	Enables XOR bit-shuffling using the Master Key (42).

    💡 Raw Text Mode: If both use_compression and use_encryption are set to false, the .token file will be saved as Raw Text. This allows you to read and edit the token content directly in any text editor without needing token.read.

📂 File Constraints & Security

    Strict Extensions: For security and consistency, only files with .token or .tk extensions are processed.

    Environment Matching: Tokens must be executed with the same global settings (compression/encryption) used during their creation.

    No Omissions: The core engine ensures that all data fields defined in config.toml are fully processed without data loss.
