use ecies::utils::generate_keypair;

pub fn test_generate_keypair() {
    let (sk1, pk1) = generate_keypair();
    let (sk2, pk2) = generate_keypair();
    assert_ne!(sk1, sk2);
    assert_ne!(pk1, pk2);
}