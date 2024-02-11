# RustyVault

RustyVault is a secure command-line password manager implemented in Rust. It provides users with the ability to generate strong, random passwords and securely store and retrieve password entries.

## Project Overview

This project is divided into two main modules: password generation and password management.

### Password Generation Module

The password generation module focuses on generating strong, random passwords according to user-defined criteria. This module ensures that users can create secure passwords for their accounts with ease.

#### Current Tasks

- [x] Set up project repository
- [ ] Implement password generation mechanism
- [ ] Write tests for password generation logic
- [ ] Design user interface for specifying password generation options
- [ ] Develop command-line interface (CLI) interaction for password generation
- [ ] Test and validate password generation functionality
- [ ] Document password generation usage and options

#### Planned Features

- [ ] Add configuration file for customization of default password generation options
- [ ] Implement password strength analysis for user-created passwords
- [ ] Enhance password generation algorithm for improved randomness and security

### Password Management Module

The password management module focuses on securely storing and managing password entries. This module ensures that users can access their passwords whenever they need them while keeping them protected from unauthorized access.

#### Current Tasks

- [x] Design data model for password entries
- [ ] Choose encryption mechanism
- [ ] Implement password encryption and storage functionality
- [ ] Develop command-line interface (CLI) interaction for password management
- [ ] Test and validate password storage and retrieval functionality
- [ ] Document password management usage and options

#### Planned Features

- [ ] Add support for storing additional metadata (e.g., website URLs, account names)
- [ ] Implement import and export functionality for password data
- [ ] Explore options for secure password sharing between trusted users

### Future Enhancements

- [ ] Integrate password generation module with password management for automatic password generation
- [ ] Implement support for multi-device synchronization and backup
- [ ] Develop browser extension(s) to automate password insertion into web forms
- [ ] Enhance usability with interactive prompts and user-friendly messaging

## Goal

RustyVault is a project aimed at expanding my skills in Rust programming and exploring best practices in encryption and decryption. While the primary goal is to create a secure command-line password manager, the project also serves as a learning opportunity for me to deepen my understanding of Rust and cryptography.

## Contributing

Contributions to RustyVault are currently not accepted as this project is primarily for educational purposes. However, feel free to fork the repository and experiment with the code for your own use. The contribution policy may change in the future as the project grows in scope.

## License

This project is licensed under the [MIT License](LICENSE).
