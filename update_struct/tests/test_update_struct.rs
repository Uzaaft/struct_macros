use update_struct::UpdateStruct;

#[test]
fn test_update_struct_macro() {
    #[derive(UpdateStruct)]
    struct User {
        id: u32,
        name: String,
        email: String,
    }
    let user_update = UpdateUser {
        name: Some("Alice".to_string()),
        email: None,
    };

    assert_eq!(user_update.name, Some("Alice".to_string()));
    assert!(user_update.email.is_none());
}
