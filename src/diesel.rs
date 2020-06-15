use diesel::backend::Backend;
use diesel::deserialize;
use diesel::deserialize::FromSqlRow;
use diesel::expression::bound::Bound;
use diesel::expression::AsExpression;
use diesel::row::Row;
use diesel::{Expression, Queryable};

use super::TaggedId;

impl<T, DB, ST> Queryable<ST, DB> for TaggedId<T>
where
    DB: Backend,
    TaggedId<T>: FromSqlRow<ST, DB>,
{
    type Row = Self;

    fn build(row: Self::Row) -> Self {
        row
    }
}

impl<T, DB, ST> FromSqlRow<ST, DB> for TaggedId<T>
where
    DB: Backend,
    String: FromSqlRow<ST, DB>,
{
    fn build_from_row<U: Row<DB>>(row: &mut U) -> deserialize::Result<Self> {
        let str = String::build_from_row(row)?;
        Ok(TaggedId::parse_str(&str)?)
    }
}

impl<T, ST> AsExpression<ST> for TaggedId<T>
where
    Bound<ST, String>: Expression<SqlType = ST>,
{
    type Expression = Bound<ST, String>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self.to_string())
    }
}

impl<'a, T, ST> AsExpression<ST> for &'a TaggedId<T>
where
    Bound<ST, String>: Expression<SqlType = ST>,
{
    type Expression = Bound<ST, String>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self.to_string())
    }
}
