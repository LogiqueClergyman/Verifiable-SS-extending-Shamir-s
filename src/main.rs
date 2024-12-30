pub mod vss;

use vss::{
    broadcast_coefficients_and_encryption_vars, broadcast_decrypted_shares, create_polynomial,
    create_shares, decrypt_received_share, encrypt_shares, reconstruct_secret,
    send_encrypted_shares, verify_received_share,
};

fn main() {
    let secret = 123456;
    let players = vec![14, 21, 32, 45, 53];
    let threshold = 3;

    // Dealer generates the polynomial
    let coefficients = create_polynomial(secret, threshold);

    // Dealer generates the shares
    let shares = create_shares(&coefficients, &players);

    // Dealer encrypts the shares (abhi toh no encryption)
    let encrypted_shares = encrypt_shares(&shares, &players);

    // Dealer sends the encrypted shares to the users
    send_encrypted_shares(&encrypted_shares);

    // Dealer broadcasts the coefficients and encryption variables
    broadcast_coefficients_and_encryption_vars(&coefficients);

    // Players verify the received shares
    println!("----------Players verifying the shares----------");
    for (i, &encrypted_share) in encrypted_shares.iter().enumerate() {
        verify_received_share(encrypted_share, &coefficients, players[i]);
    }
    println!("----------Shares verified by all players----------");

    // Players decrypt the received shares (since no encryption, no decryption, at least abhi ke liye toh nhi)
    let mut decrypted_shares: Vec<(i32, i32)> = Vec::new();
    for (i, &encrypted_share) in encrypted_shares.iter().enumerate() {
        decrypted_shares.push((players[i], decrypt_received_share(encrypted_share)));
    }

    // Players broadcast the decrypted shares along with the index
    broadcast_decrypted_shares(&decrypted_shares);

    // Secret is reconstructed
    let reconstructed_secret = reconstruct_secret(&decrypted_shares, threshold);
    println!("Reconstructed secret: {}", reconstructed_secret);
}
