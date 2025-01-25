#[cfg(test)]
mod tests {
    use nostro2::{keypair::NostrKeypair, notes::NostrNote};

    use crate::GrpcNostrNote;
    #[test]
    fn it_works() {
        let new_users = NostrKeypair::generate(false);
        let pubkey = new_users.public_key();
        let mut new_note = NostrNote {
            pubkey,
            kind: 1,
            content: "Test note".to_string(),
            ..Default::default()
        };
        new_users
            .sign_nip_44_encrypted(&mut new_note, new_users.public_key())
            .expect("Failed to sign note");
        println!("{}", serde_json::to_string_pretty(&new_note).unwrap());
        let grpc_note: GrpcNostrNote = new_note.clone().into();
        println!("{}", serde_json::to_string_pretty(&grpc_note).unwrap());
        assert_eq!(grpc_note.pubkey, new_users.public_key());
        let back_to_signed_note: NostrNote = grpc_note.into();
        println!("{}", serde_json::to_string_pretty(&back_to_signed_note).unwrap());
        assert_eq!(back_to_signed_note, new_note);
    }
}
