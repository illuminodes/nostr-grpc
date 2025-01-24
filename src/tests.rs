#[cfg(test)]
mod tests {
    use nostro2::{notes::{Note, NostrNote}, keypair::NostrKeypair};

    use crate::GrpcNostrNote;
    #[test]
    fn it_works() {
        let new_users = UserKeys::generate();
        let pubkey = new_users.get_public_key();
        let new_note = Note::new(&pubkey, 1, "Test note");
        let signed_note = new_users
            .sign_nip_44_encrypted(new_note, pubkey)
            .expect("Failed to sign note");
        println!("{}", serde_json::to_string_pretty(&signed_note).unwrap());
        let grpc_note: GrpcNostrNote = signed_note.clone().into();
        println!("{}", serde_json::to_string_pretty(&grpc_note).unwrap());
        assert_eq!(grpc_note.pubkey, new_users.get_public_key());
        let back_to_signed_note: NostrNote = grpc_note.into();
        assert_eq!(back_to_signed_note, signed_note);
    }
}
