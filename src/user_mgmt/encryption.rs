use bcrypt::DEFAULT_COST;

use super::error::AuthError;

// consume password value to make it unusable
pub async fn hash(password: String) -> Result<String, AuthError> {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result = bcrypt::hash(password, DEFAULT_COST);
        let _ = send.send(result);
    });
    Ok(recv.await??)
}

pub async fn verify(password: String, hash: String) -> Result<bool, AuthError> {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result = bcrypt::verify(password, &hash);
        let _ = send.send(result);
    });
    Ok(recv.await??)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let hashed = hash("hunter2".to_string()).await.unwrap();
        let valid = verify("hunter2".to_string(), hashed).await.unwrap();
        assert!(valid)
    }
}
