use crate::{autentifications::auth::Auth, users::user::User};

#[tokio::test]
async fn user_validate() {
    let test_user: User = User {
        name: "Example".to_owned(),
        password: "StrongPass".to_owned(),
        email: "Some@email.xxx".to_owned(),
        role: crate::users::user::Role::Default,
    };

    let another_user: User = User {
        name: "Example".to_owned(),
        password: "StrongPass".to_owned(),
        email: "Some@email.xxx".to_owned(),
        role: crate::users::user::Role::Default,
    };

    assert_eq!(true, test_user.validate(&another_user))
}

#[tokio::test]
#[should_panic]
async fn user_validate_panic() {
    let test_user: User = User {
        name: "Example".to_owned(),
        password: "StrongPass".to_owned(),
        email: "Some@email.xxx".to_owned(),
        role: crate::users::user::Role::Default,
    };

    let another_user: User = User {
        name: "Example".to_owned(),
        password: "WRONG_PASSWORD".to_owned(),
        email: "Some@email.xxx".to_owned(),
        role: crate::users::user::Role::Default,
    };

    assert_eq!(true, test_user.validate(&another_user))
}

#[tokio::test]
async fn user_validate_anonimus() {
    let test_user: User = User {
        name: "Example".to_owned(),
        password: "StrongPass".to_owned(),
        email: "Some@email.xxx".to_owned(),
        role: crate::users::user::Role::Default,
    };

    let another_user: Auth = Auth {
        name: "Example".to_owned(),
        password: "StrongPass".to_owned(),
    };
    assert_eq!(true, test_user.validate_anonimus(&another_user))
}

#[tokio::test]
#[should_panic]
async fn user_validate_anonimus_panic() {
    let test_user: User = User {
        name: "Example".to_owned(),
        password: "StrongPass".to_owned(),
        email: "Some@email.xxx".to_owned(),
        role: crate::users::user::Role::Default,
    };

    let another_user: Auth = Auth {
        name: "Example".to_owned(),
        password: "WRONG_PASSWORD".to_owned(),
    };
    assert_eq!(true, test_user.validate_anonimus(&another_user))
}

#[tokio::test]
async fn auth_new() {
    let auth = Auth::new("Example".to_owned(), "StrongPass".to_owned());
    let true_auth = Auth {
        name: "Example".to_owned(),
        password: "StrongPass".to_owned(),
    };
    assert_eq!(true_auth, auth)
}

#[tokio::test]
//it's will panic cause we don't have running server to test this
#[should_panic]
async fn auth_validate() {
    let auth = Auth::new("Example".to_owned(), "StrongPass".to_owned());
    let auth = auth.validate().await;
    assert_eq!(true, auth)
}
