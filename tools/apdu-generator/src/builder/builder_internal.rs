pub fn fix(hash: &mut String) {
    // fix pedersen hash to fit into 32 bytes
    while hash.len() < 63 {
        hash.insert(0, '0');
    }
    assert!(hash.len() == 63);
    hash.push('0');
}