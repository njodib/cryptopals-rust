use crate::aes_algs::{aes_ecb_encrypt, aes_ecb_decrypt};

const KEY: [u8; 16] = *b"COUNTY FAIR THOT";

pub fn profile_for(email: &[u8]) -> Vec<u8> {
    let mut user: Vec<u8> = b"email=".to_vec();
    email
        .to_vec()
        .into_iter()
        .for_each(|b| if b!=b'&' && b!=b'=' {user.push(b)});
    user.append(&mut b"&uid=10&role=user".to_vec());
    aes_ecb_encrypt(&user, &KEY)
}

pub fn print() {
    //10 bytes + "admin" + 11 bytes
    let admin_email = [
        &[10u8;10],
        "admin".as_bytes(),
        &[11u8;11]
    ].concat();

    //end block of admin profile will have "admin" + end padding
    let admin_block = profile_for(&admin_email)
        .chunks(16)
        .nth(1)
        .unwrap()
        .to_vec();

    //create a normal user profile, take everything except value of 'role'
    let user_profile = profile_for(b"fish@fish.com") //13 bytes
        .chunks(16)
        .take(2)
        .collect::<Vec<&[u8]>>()
        .concat();

    //concat the user profile with encrypted block: 'admin' + end padding
    let admin_profile = [
        user_profile.clone(),
        admin_block.clone()
    ].concat();

    //use lossy to deal with non ASCII-standard end paddings
    println!("admin profile: {:?}", 
        String::from_utf8_lossy(
            &aes_ecb_decrypt(&admin_profile, &KEY)
        )
    )
}
