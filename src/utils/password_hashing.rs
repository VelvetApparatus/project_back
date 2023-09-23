use argon2::{Argon2, PasswordHash,  password_hash::SaltString, PasswordHasher};


pub fn hash(password: &[u8], id: String) -> argon2::password_hash::Output {

    // let salt = SaltString::generate(&mut OsRng);


    // Changing Uuid for Salt requirements 
    let id_t = id.clone().replace("-", "");
        
    // Make Salt based on Uuid
    let salt = SaltString::from_b64(&id_t).expect("BAD SALT");
    
    // Argon Announcement 
    let argon2 = Argon2::default();

    // Hashing
    let password_hash = argon2.hash_password(password, &salt).unwrap_or_else(|e| panic!("Error detected in hashing service: {:?} ", e)).to_string();
    
    // Make more useful struct
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();


    let res = parsed_hash.hash.unwrap();
    
    return res
}