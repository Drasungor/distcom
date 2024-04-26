// use diesel::prelude::*;

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
        username -> Varchar, // Set as unique in migration

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

// A group represents a set of inputs reserved for a specific program, inputs
// are classified as a group because all the inputs used for a program are either
// used at the same time or not at all, a program can have multiple input groups
diesel::table! {
    program (program_id) {

        #[max_length = 255]
        organization_id -> Varchar,

        #[max_length = 255]
        program_id -> Varchar,

        input_lock_timeout -> BigInt,
    }
}

// A group represents a set of inputs reserved for a specific program, inputs
// are classified as a group because all the inputs used for a program are either
// used at the same time or not at all, a program can have multiple input groups
diesel::table! {
    program_input_group (input_group_id) {

        #[max_length = 255]
        input_group_id -> Varchar,

        #[max_length = 255]
        program_id -> Varchar,

        // // Determines if this group was provided to a prover
        // input_was_reserved -> Bool,

        // last_reserved -> Timestamp,
        last_reserved -> Nullable<Datetime>,
    }
}

// Where the specific groups are stored, order is important because inputs are inserted sequentially
// inside the prover. Inputs have a size limit, but since the insertion is sequential any object that
// does not fit inside the limit can be separated in multiple parts
diesel::table! {
    specific_program_input (specific_input_id) {

        #[max_length = 255]
        specific_input_id -> Varchar,

        #[max_length = 255]
        input_group_id -> Varchar, // Generate index in migration

        #[max_length = 1024]
        blob_data -> Nullable<Varbinary>,

        order -> Integer,
    }
}


// diesel::allow_tables_to_appear_in_same_query!(
//     account, 
//     papafrita,
//     program_input
// );

// Add a unique index to ensure username uniqueness


// sql_query!("CREATE UNIQUE INDEX idx_username_unique ON account (username);");