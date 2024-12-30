use rand::Rng;

pub fn create_polynomial(secret: i32, threshold: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut coefficients: Vec<i32> = Vec::new();
    coefficients.push(secret);
    for _ in 1..threshold {
        coefficients.push(rng.gen_range(0..100));
    }
    coefficients
}

pub fn create_shares(coefficients: &Vec<i32>, players: &Vec<i32>) -> Vec<i32> {
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

pub fn encrypt_shares(shares: &Vec<i32>, _players: &Vec<i32>) -> Vec<i32> {
    shares.clone() // No encryption for now
}

pub fn send_encrypted_shares(encrypted_shares: &Vec<i32>) {
    for (i, &share) in encrypted_shares.iter().enumerate() {
        println!("Share sent to player {}: {}", i + 1, share);
    }
    println!("----------Shares sent to all players privately----------");
}

pub fn broadcast_coefficients_and_encryption_vars(coefficients: &Vec<i32>) {
    println!("Coefficients: {:?}", coefficients);
    println!("----------Coefficients and encryption variables broadcasted----------");
}

pub fn verify_received_share(
    received_encrypted_share: i32,
    coefficients: &Vec<i32>,
    player_index: i32,
) {
    let mut expected_share = 0;
    for (i, &coeff) in coefficients.iter().enumerate() {
        expected_share += coeff * player_index.pow(i as u32);
    }
    match received_encrypted_share == expected_share {
        true => println!("Share verified by player {}", player_index),
        false => {
            println!(
                "Player {} received a faulty share: Expected {}, Got {}",
                player_index, expected_share, received_encrypted_share
            );
            panic!("Dealer is corrupt");
        }
    }
}

pub fn decrypt_received_share(received_encrypted_share: i32) -> i32 {
    received_encrypted_share // No decryption for now
}

pub fn broadcast_decrypted_shares(decrypted_shares: &Vec<(i32, i32)>) {
    for &(index, share) in decrypted_shares {
        println!(
            "Player {} broadcast: [index={} | share={}]",
            index, index, share
        );
    }
    println!("----------Decrypted shares broadcasted----------");
}

pub fn reconstruct_secret(decrypted_shares: &Vec<(i32, i32)>, threshold: usize) -> i32 {
    let mut secret = 0;
    for j in 0..threshold {
        let (xj, yj) = decrypted_shares[j];
        let mut numerator = 1;
        let mut denominator = 1;
        for k in 0..threshold {
            if j != k {
                let (xk, _) = decrypted_shares[k];
                numerator *= -xk;
                denominator *= xj - xk;
            }
        }
        secret += yj * numerator / denominator;
    }
    secret
}
