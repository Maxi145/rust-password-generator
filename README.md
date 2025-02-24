# Password Generator 🦀

A simple and efficient password generator built with Rust and Egui. Generate secure passwords with customizable options for uppercase letters, lowercase letters, numbers, and symbols.

## Features

- Generate passwords with customizable length (4-20 characters)
- Include or exclude uppercase letters, lowercase letters, numbers, and symbols
- Generate multiple passwords at once (up to 50)
- Copy individual passwords or all generated passwords to the clipboard
- User-friendly interface with clear instructions and icons

## Screenshots
### **Main Interface**
![Main Interface](https://github.com/user-attachments/assets/6452c7df-ac72-438c-9a3e-fa80ba013e73)

### **Generated Passwords**

![Generated Passwords](https://github.com/user-attachments/assets/61cc5fbd-c176-42f5-adaf-6c29df9c48b4)

## How to use
1. Download and Install:
   - Download the latest release from the [Releases](https://github.com/Maxi145/rust-password-generator/releases) page. Choose the appropriate version for your system (x64 or x86).

3. Run the Application:
   - Double-click the downloaded executable to run the password generator.

4. Customize Settings:

   - Adjust the password length using the slider.
   - Check or uncheck the options to include uppercase letters, lowercase letters, numbers, and symbols.
   - Set the number of passwords to generate.

5. Generate Passwords:
   - Click the "🔄 Generate Passwords" button to generate the passwords.

6. Copy Passwords:

   - Left-click on an individual password to copy it to the clipboard.
   - Click the "📋 Copy All Passwords" button to copy all generated passwords to the clipboard.

## Building from Source
To build the password generator from source, follow these steps:

1. Clone the repository:
   ```sh
   git clone https://github.com/Maxi145/rust-password-generator.git
   cd password-generator
2. Install Rust Toolchain: Ensure you have the necessary Rust toolchains installed:
   ```sh
   rustup target add x86_64-pc-windows-msvc
   rustup target add i686-pc-windows-msvc
3. Build the Project:
      ```sh
   # Build for x64
   cargo build --release --target x86_64-pc-windows-msvc
   
   # Build for x32
   cargo build --release --target i686-pc-windows-msvc
4. Run the Application:
   ```sh
   ./target/x86_64-pc-windows-msvc/release/password-generator.exe
   
