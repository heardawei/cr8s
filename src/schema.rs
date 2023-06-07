diesel::table! {
  crates (id) {
    id -> Integer,
    rustacean_id -> Integer,
    code -> Varchar,
    name -> Varchar,
    version -> Varchar,
    description -> Nullable<Text>,
    create_at -> Timestamp,
  }
}

diesel::table! {
  rustaceans (id) {
    id -> Integer,
    name -> VarChar,
    email -> VarChar,
    create_at -> Timestamp,
  }
}

diesel::joinable!(crates -> rustaceans (rustacean_id));

diesel::allow_tables_to_appear_in_same_query!(
  crates,
  rustaceans,
);
