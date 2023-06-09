use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table
            .limit(limit)
            .order(rustaceans::id.desc())
            .load::<Rustacean>(c)
    }

    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result::<Rustacean>(c)
    }

    pub fn create(c: &mut PgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(c)
    }

    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }

    pub fn save(c: &mut PgConnection, id: i32, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(new_rustacean.name.to_owned()),
                rustaceans::email.eq(new_rustacean.email.to_owned()),
            ))
            .execute(c)?;

        Self::find(c, id)
    }
}

pub struct CrateRepository;

impl CrateRepository {
    pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table
            .limit(limit)
            .order(crates::id.desc())
            .load::<Crate>(c)
    }

    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result::<Crate>(c)
    }

    pub fn create(c: &mut PgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result::<Crate>(c)
    }

    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(c)
    }

    pub fn save(c: &mut PgConnection, id: i32, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::name.eq(new_crate.name.to_owned()),
                crates::code.eq(new_crate.code.to_owned()),
                crates::version.eq(new_crate.version.to_owned()),
                crates::description.eq(new_crate.description.to_owned()),
                crates::rustacean_id.eq(new_crate.rustacean_id.to_owned()),
            ))
            .execute(c)?;

        Self::find(c, id)
    }
}
