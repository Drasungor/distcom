diesel::table! {
    account (organization_id) {
        organization_id -> BigInt,
        name -> Varchar,
        description -> Varchar,
        published -> Bool,
    }
}

// diesel::table! {
//     posts (id) {
//         id -> Int4,
//         title -> Varchar,
//         body -> Text,
//         published -> Bool,
//     }
// }