# Discord Encrypted Storage Explorer

**Discord Encrypted Storage Explorer** is a Rust-based tool for exploring **your own local Discord data**. It demonstrates how encrypted data is stored and secured, and how Windows DPAPI and AES-GCM encryption work.

> ⚠️ **Disclaimer:** This project is for educational purposes only. Use it **only on your own Discord account** to understand local encrypted storage. Do **not** use it to access others’ accounts. Respect Discord’s Terms of Service.

---

## Table of Contents

* [Features](#features)
* [Installation](#installation)
* [Usage](#usage)
* [Project Structure](#project-structure)
* [Contributing](#contributing)
* [License](#license)

---

## Features

* **Educational Exploration:** Learn how Discord stores encrypted tokens locally.
* **Local Storage Reading:** Explore your own Discord encrypted data for learning purposes.
* **DPAPI & AES-GCM Demonstration:** Understand Windows DPAPI key decryption and AES-GCM token encryption.
* **Debug-Friendly:** Detailed debug logs for educational insights.

---

## Installation

### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) (latest stable)
* Windows OS (for DPAPI decryption)

### Clone the Repository

```bash
git clone https://github.com/obfusque/discord-encrypted-storage-explorer.git
cd discord-encrypted-explorer
```

### Build the Project

```bash
cargo build --release
```

---

## Usage

1. Set the base path to your Discord roaming data:

```rust
let base_path = Path::new(r"C:\Users\<YourUser>\AppData\Roaming\Discord");
```

2. Run the project:

```bash
cargo run
```

3. The tool will scan your local Discord storage and print **educational decrypted entries** to the console.

> **Important:** Only use this on your **own Discord account**. Accessing other users’ data is illegal and violates Discord’s Terms of Service.

---

## Project Structure

```
src/
├─ main.rs       # Entry point
├─ tokens.rs     # Extracts encrypted storage entries
├─ crypto.rs     # Handles DPAPI & AES-GCM decryption
Cargo.toml
```

---

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a branch: `git checkout -b feature/your-feature`
3. Make changes and commit: `git commit -m "Add feature"`
4. Push to your branch: `git push origin feature/your-feature`
5. Open a pull request.

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

