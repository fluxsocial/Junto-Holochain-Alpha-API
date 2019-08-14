table! {
    users (id) {
        id -> Uuid,
        email -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        pub_address -> Nullable<Varchar>,
    }
}
