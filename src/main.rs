use rand::Rng;

fn main() {
    let secret = 123;
    let players = vec![1, 2, 3, 4, 5];
    let threshold = 3;

    // Dealer generates the polynomial
    let coefficients = create_polynomial(secret, threshold);

    //Dealer generates the shares
    let shares = create_shares(&coefficients, &players);
    
    // Dealer encrypts the shares
    // let encrypted_shares = encrypt_shares();

    // Dealer sends the encrypted shares to the users
    // send_encrypted_shares();

    // Dealer broadcasts the coefficients and encryption variables
    // broadcast_coefficients_and_encryption_vars();

    // Players verify the received shares
    // verify_received_shares();

    // Players decrypt the received shares
    // let decrypted_shares = decrypt_received_shares();

    // Players broadcast the decrypted shares along with the index
    // broadcast_decrypted_shares();

    // Secret is reconstructed
    // let reconstructed_secret = reconstruct_secret();
    // println!("Reconstructed secret: {}", reconstructed_secret);
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
    for player in players {
        let mut share = 0;
        for i in 0..coefficients.len() {
            share += coefficients[i] * player.pow(i as u32);
        }
        shares.push((share));
    }
    shares
}

fn encrypt_shares(coefficients: &[i32], players: &[u8]) -> Vec<(u8, i32)> {
    let mut encrypted_shares: Vec<(u8, i32)> = Vec::new();
    for i in 
    encrypted_shares
}

fn send_encrypted_shares(encrypted_shares: &[(u8, i32)]) {}

fn broadcast_coefficients_and_encryption_vars(polynomial: &[i32]) {}

fn verify_received_shares(encrypted_shares: &[(u8, i32)], polynomial: &[i32]) {}

fn decrypt_received_shares(encrypted_shares: &[(u8, i32)]) -> Vec<(u8, i32)> {
    Vec::new()
}

fn broadcast_decrypted_shares(decrypted_shares: &[(u8, i32)]) {}

fn reconstruct_secret(shares: &[(u8, i32)], threshold: usize) -> i32 {
    0
}
