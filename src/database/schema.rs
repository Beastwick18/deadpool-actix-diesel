diesel::table! {
    users (id) {
        id -> BigInt,
        first_name -> Varchar,
        last_name -> Varchar,
        ssn -> Varchar,
        email -> Nullable<Varchar>,
    }
}
