use diesel::{Expression, Queryable};
use diesel::backend::Backend;
use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;
use diesel::expression::bound::Bound;

use super::*;

impl<DB, In, ST> Queryable<ST, DB> for TaggedId<In>
    where
        DB: Backend,
        TaggedId<In>: FromSqlRow<ST, DB>
{
    type Row = Self;

    fn build(row: Self::Row) -> Self {
        row
    }
}

impl<DB: Backend, In, ST> FromSqlRow<ST, DB> for TaggedId<In>
    where
        String: FromSqlRow<ST, DB>
{
    fn build_from_row<T: diesel::row::Row<DB>>(row: &mut T) -> diesel::deserialize::Result<Self> {
        let str = String::build_from_row(row)?;
        let id = Uuid::parse_str(&str)?;
        Ok(TaggedId::from_uuid(id))
    }
}

impl<T, ST> AsExpression<ST> for TaggedId<T>
    where
        Bound<ST, String>: Expression<SqlType=ST>
{
    type Expression = Bound<ST, String>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self.inner().to_string())
    }
}

impl<'a, T, ST> AsExpression<ST> for &'a TaggedId<T>
    where
        Bound<ST, String>: Expression<SqlType=ST>
{
    type Expression = Bound<ST, String>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self.inner().to_string())
    }
}