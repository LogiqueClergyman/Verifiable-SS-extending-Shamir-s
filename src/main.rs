use rand::Rng;

fn main() {
    let secret = 123;
    let players = vec![1, 2, 3, 4, 5];
    let threshold = 3;

    // Dealer generates the polynomial
    let coefficients = create_polynomial(secret, threshold);

    // Dealer generates the shares
    let shares = create_shares(&coefficients, &players);

    // Dealer encrypts the shares (no actual encryption for now)
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

    // Players decrypt the received shares
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

fn create_polynomial(secret: i32, threshold: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut coefficients: Vec<i32> = Vec::new();
    coefficients.push(secret);
    for _ in 1..threshold {
        coefficients.push(rng.gen_range(0..100));
    }
    coefficients
}

fn create_shares(coefficients: &Vec<i32>, players: &Vec<i32>) -> Vec<i32> {
    let mut shares: Vec<i32> = Vec::new();
    for &player in players {
        let mut share = 0;
        for (i, &coeff) in coefficients.iter().enumerate() {
            share += coeff * player.pow(i as u32);
        }
        shares.push(share);
    }
    shares
}

fn encrypt_shares(shares: &Vec<i32>, _players: &Vec<i32>) -> Vec<i32> {
    shares.clone() // No encryption for now
}

fn send_encrypted_shares(encrypted_shares: &Vec<i32>) {
    for (i, &share) in encrypted_shares.iter().enumerate() {
        println!("Share sent to player {}: {}", i + 1, share);
    }
    println!("----------Shares sent to all players privately----------");
}

fn broadcast_coefficients_and_encryption_vars(coefficients: &Vec<i32>) {
    println!("Coefficients: {:?}", coefficients);
    println!("----------Coefficients and encryption variables broadcasted----------");
}

fn verify_received_share(
    received_encrypted_share: i32,
    coefficients: &Vec<i32>,
    player_index: i32,
) {
    let mut expected_share = 0;
    for (i, &coeff) in coefficients.iter().enumerate() {
        expected_share += coeff * player_index.pow(i as u32);
    }
    if received_encrypted_share == expected_share {
        println!("Share verified by player {}", player_index);
    } else {
        println!(
            "Player {} received a faulty share: Expected {}, Got {}",
            player_index, expected_share, received_encrypted_share
        );
        panic!("Dealer is corrupt");
    }
}

fn decrypt_received_share(received_encrypted_share: i32) -> i32 {
    received_encrypted_share // No decryption for now
}

fn broadcast_decrypted_shares(decrypted_shares: &Vec<(i32, i32)>) {
    for &(index, share) in decrypted_shares {
        println!("Player {} broadcast: [index={} | share={}]", index, index, share);
    }
    println!("----------Decrypted shares broadcasted----------");
}

fn reconstruct_secret(decrypted_shares: &Vec<(i32, i32)>, threshold: usize) -> i32 {
    let mut secret = 0;
    for (j, &(xj, yj)) in decrypted_shares.iter().take(threshold).enumerate() {
        let mut numerator = 1;
        let mut denominator = 1;
        for (k, &(xk, _)) in decrypted_shares.iter().take(threshold).enumerate() {
            if j != k {
                numerator *= -xk;
                denominator *= xj - xk;
            }
        }
        secret += yj * numerator / denominator;
    }
    secret
}
