table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        bio -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        password -> Varchar,
    }
}
