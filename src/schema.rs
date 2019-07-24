table! {
    users (id) {
        id -> Uuid,
        email -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        agent_key -> Nullable<Varchar>,
    }
}
