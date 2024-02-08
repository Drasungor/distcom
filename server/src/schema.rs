// @generated automatically by Diesel CLI.

diesel::table! {
    account (organization_id) {
        organization_id -> Bigint,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        account_was_verified -> Bool,
    }
}
