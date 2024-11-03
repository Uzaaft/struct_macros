// Import the macro from your crate
use new_struct::NewStruct;

#[test]
fn test_remove_id_macro() {
    // Define a struct with an `id` field
    #[derive(NewStruct)]
    struct User {
        id: u32,
        name: String,
        email: String,
    }

    // After the macro is applied, we expect a `NewUser` struct to be generated
    let new_user = NewUser {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    // Ensure that `NewUser` has the fields we expect
    assert_eq!(new_user.name, "Alice");
    assert_eq!(new_user.email, "alice@example.com");
}
