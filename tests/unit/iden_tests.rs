use crate::*;
use Character as CharReexport;

enum Character {
    Table,
    Id,
}

type Char = Character;

impl Iden for Character {
    fn unquoted(&self) -> &str {
        match self {
            Self::Table => "character",
            Self::Id => "id",
        }
    }
}

enum Font {
    Id,
}

impl Iden for Font {
    fn unquoted(&self) -> &str {
        "id"
    }
}

#[test]
fn test_identifier() {
    let query = Query::select().column("hello-World_").to_owned();

    assert_eq!(
        query.to_string(MysqlQueryBuilder).unwrap(),
        r"SELECT `hello-World_`"
    );

    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        r#"SELECT "hello-World_""#
    );

    assert_eq!(
        query.to_string(SqliteQueryBuilder).unwrap(),
        r#"SELECT "hello-World_""#
    );
}

#[test]
fn test_quoted_identifier_1() {
    let query = Query::select().column("hel`lo").to_owned();

    assert_eq!(
        query.to_string(MysqlQueryBuilder).unwrap(),
        r"SELECT `hel``lo`"
    );

    assert_eq!(
        query.to_string(SqliteQueryBuilder).unwrap(),
        r#"SELECT "hel`lo""#
    );

    let query = Query::select().column("hel\"lo").to_owned();

    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        r#"SELECT "hel""lo""#
    );
}

#[test]
fn test_quoted_identifier_2() {
    let query = Query::select().column("hel``lo").to_owned();

    assert_eq!(
        query.to_string(MysqlQueryBuilder).unwrap(),
        r"SELECT `hel````lo`"
    );

    assert_eq!(
        query.to_string(SqliteQueryBuilder).unwrap(),
        r#"SELECT "hel``lo""#
    );

    let query = Query::select().column("hel\"\"lo").to_owned();

    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        r#"SELECT "hel""""lo""#
    );
}

#[test]
fn test_cmp_identifier() {
    type CharLocal = Character;

    assert_eq!(
        ColumnRef::Column(Character::Id.into()),
        ColumnRef::Column(Character::Id.into())
    );
    assert_eq!(
        ColumnRef::Column(Character::Id.into()),
        ColumnRef::Column(Char::Id.into())
    );
    assert_eq!(
        ColumnRef::Column(Character::Id.into()),
        ColumnRef::Column(CharLocal::Id.into())
    );
    assert_eq!(
        ColumnRef::Column(Character::Id.into()),
        ColumnRef::Column(CharReexport::Id.into())
    );
    assert_eq!(
        ColumnRef::Column("id".into()),
        ColumnRef::Column("id".into())
    );
    assert_ne!(
        ColumnRef::Column("id".into()),
        ColumnRef::Column("id_".into())
    );
    assert_eq!(
        ColumnRef::Column(Character::Id.into()),
        ColumnRef::Column("id".into())
    );
    assert_ne!(
        ColumnRef::Column(Character::Id.into()),
        ColumnRef::Column(Character::Table.into())
    );
    assert_eq!(
        ColumnRef::Column(Character::Id.into()),
        ColumnRef::Column(Font::Id.into())
    );
}
