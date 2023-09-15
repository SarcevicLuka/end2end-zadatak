// @generated automatically by Diesel CLI.

diesel::table! {
    employees (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 1]
        sex -> Varchar,
        image -> Bytea,
        birth_year -> Int4,
        #[max_length = 20]
        start_of_work -> Varchar,
        #[max_length = 10]
        type_of_contract -> Varchar,
        #[max_length = 20]
        length_of_contract -> Varchar,
        #[max_length = 50]
        department -> Varchar,
        days_of_holiday -> Nullable<Int4>,
        free_days -> Nullable<Int4>,
        days_of_paid_leave -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
