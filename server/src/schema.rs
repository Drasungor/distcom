// @generated automatically by Diesel CLI.

diesel::table! {
    account (organization_id) {
        
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

        #[max_length = 255]
        token_id -> Varchar,

        #[max_length = 255]
        user_id -> Varchar,

    }
}

// diesel::allow_tables_to_appear_in_same_query!(
//     account, 
//     papafrita,
// );
