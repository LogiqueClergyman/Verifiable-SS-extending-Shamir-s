# Verifiable Secret Sharing (VSS)

## Table of Contents

- [About](#about)
- [Background](#background)
- [Explaination](#getting_started)
- [Prerequisites](#prerequisites)
- [Running](#usage)

## About <a name = "about"></a>

Verifiable Secret Sharing extending Shamir's Secret Sharing algo. It is a cryptographic protocol that allows a dealer to share a secret among a group of participants, such that the secret can be reconstructed only when a sufficient number of participants collaborate. 

This implementation is based on the paper [Verifiable Secret Sharing and Multiparty Protocols with Honest Majority](https://www.cs.cornell.edu/courses/cs754/2001fa/129.pdf) by Silvio Micali, Michael Rabin, and Salil Vadhan.

## Background <a name = "background"></a>

Shamir's Secret Sharing (SSS) is a cryptographic algorithm that divides a secret into multiple parts (shares), where a specified minimum number of shares are required to reconstruct the original secret. It uses polynomial interpolation, ensuring that any subset of shares smaller than the threshold reveals no information about the secret.

Verifiable Secret Sharing (VSS) enhances SSS by adding verification capabilities. It allows participants to verify that their shares are consistent with each other, preventing malicious dealers from distributing invalid shares. This verification mechanism ensures the reliability of the secret sharing process while maintaining the security properties of the original SSS scheme.

## Explaination <a name = "Explaination"></a>

The code implements a Verifiable Secret Sharing (VSS) scheme based on Shamir's Secret Sharing algorithm. Here is a detailed explanation of the flow:

1. **Initialization**:
   - The `main` function initializes the secret, the list of players, and the threshold value.
   - The secret is the value to be shared, players are the participants, and the threshold is the minimum number of shares needed to reconstruct the secret.

2. **Polynomial Creation**:
   - The `create_polynomial` function generates a random polynomial with the secret as the constant term and random coefficients for the remaining terms. The degree of the polynomial is one less than the threshold.

3. **Share Creation**:
   - The `create_shares` function evaluates the polynomial at different points (player indices) to generate shares. Each share is a point on the polynomial.

4. **Encryption (Placeholder)**:
   - The `encrypt_shares` function is a placeholder for encryption. Currently, it just clones the shares without any encryption.

5. **Share Distribution**:
   - The `send_encrypted_shares` function simulates sending the encrypted shares to the players by printing them.

6. **Broadcast Coefficients**:
   - The `broadcast_coefficients_and_encryption_vars` function broadcasts the polynomial coefficients. This step is crucial for verification.

7. **Share Verification**:
   - Each player verifies their received share using the `verify_received_share` function. It checks if the received share matches the expected value calculated using the polynomial coefficients.

8. **Decryption (Placeholder)**:
   - The `decrypt_received_share` function is a placeholder for decryption. Currently, it just returns the received share as is.

9. **Broadcast Decrypted Shares**:
   - The `broadcast_decrypted_shares` function simulates broadcasting the decrypted shares by printing them.

10. **Secret Reconstruction**:
    - The `reconstruct_secret` function reconstructs the secret using the decrypted shares and Lagrange interpolation. It calculates the secret by combining the shares according to the Lagrange basis polynomials.

### Prerequisites <a name = "prerequisites"></a>

What things you need to install the software and how to install them.

```
Rust
```

## Running <a name = "usage"></a>

```
cargo run
```
