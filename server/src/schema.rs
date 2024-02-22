// @generated automatically by Diesel CLI.

diesel::table! {
    account (organization_id) {
        // organization_id -> Bigint,
        #[max_length = 255]
        organization_id -> Varchar,
        
        #[max_length = 255]
        name -> Varchar,
        
        #[max_length = 255]
        description -> Varchar,
        
        account_was_verified -> Bool,
        
        #[max_length = 255]
        username -> Varchar,

        #[max_length = 255]
        password_hash -> Varchar,
    }
}

diesel::table! {
    refresh_token (token_id) {
        token_id -> Bigint,

        user_id -> Bigint,
    }
}

// diesel::allow_tables_to_appear_in_same_query!(
//     account, 
//     papafrita,
// );
