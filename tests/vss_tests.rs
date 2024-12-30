#[path = "../src/vss.rs"]
mod vss;
use vss::{create_polynomial, create_shares, reconstruct_secret, verify_received_share};

#[test]
fn test_create_polynomial() {
    let secret = 123;
    let threshold = 3;
    let coefficients = create_polynomial(secret, threshold);
    assert_eq!(coefficients.len(), threshold); // Ensure that the polynomial has the expected number of coefficients
    assert_eq!(coefficients[0], secret); // The first coefficient should be the secret
}

#[test]
fn test_create_shares() {
    let secret = 123;
    let threshold = 3;
    let players = vec![14, 21, 32, 45, 53];
    let coefficients = create_polynomial(secret, threshold);
    let shares = create_shares(&coefficients, &players);
    assert_eq!(shares.len(), players.len()); // Ensure the number of shares matches the number of players
}

#[test]
fn test_verify_received_share_valid() {
    let secret = 123;
    let threshold = 3;
    let players = vec![14, 21, 32, 45, 53];
    let coefficients = create_polynomial(secret, threshold);
    let shares = create_shares(&coefficients, &players);
    let encrypted_share = shares[0]; // Get the first player's share
    let player_index = players[0];

    // This should not panic if the share is correct
    verify_received_share(encrypted_share, &coefficients, player_index);
}

#[test]
#[should_panic(expected = "Dealer is corrupt")]
fn test_verify_received_share_invalid() {
    let secret = 123;
    let threshold = 3;
    let players = vec![14, 21, 32, 45, 53];
    let coefficients = create_polynomial(secret, threshold);
    let shares = create_shares(&coefficients, &players);
    let encrypted_share = shares[0]; // Get the first player's share
    let player_index = players[0] + 1; // Intentionally using the wrong player index

    // This should panic since the share is incorrect
    verify_received_share(encrypted_share, &coefficients, player_index);
}

#[test]
fn test_reconstruct_secret() {
    let secret = 123;
    let threshold = 3;
    let players = vec![14, 21, 32, 45, 53];
    let coefficients = create_polynomial(secret, threshold);
    let shares = create_shares(&coefficients, &players);

    // Create decrypted shares with player index and share
    let decrypted_shares: Vec<(i32, i32)> = players
        .iter()
        .take(threshold)
        .zip(shares.iter())
        .map(|(&player, &share)| (player, share))
        .collect();

    let reconstructed_secret = reconstruct_secret(&decrypted_shares, threshold);
    assert_eq!(reconstructed_secret, secret); // Ensure the reconstructed secret matches the original secret
}
