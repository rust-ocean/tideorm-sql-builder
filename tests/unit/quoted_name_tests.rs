use crate::*;

#[test]
fn constraint_and_enum_cast_names_are_escaped() {
    let sql = Query::insert()
        .into_table("t")
        .columns(["v"])
        .values_panic([1.into()])
        .on_conflict(OnConflict::constraint("c\"x").do_nothing().to_owned())
        .to_string(PostgresQueryBuilder)
        .unwrap();
    assert!(sql.contains("ON CONSTRAINT \"c\"\"x\""));
    let query = Query::select()
        .expr(Expr::val(1).as_enum("ty\"pe[]"))
        .to_owned();
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        "SELECT CAST(1 AS \"ty\"\"pe\"[])"
    );
    let value = Value::Enum(OptionEnum::Some(Box::new(Enum {
        type_name: "ty\"pe".into(),
        value: "one".into(),
    })));
    let query = Query::select().expr(Expr::val(value)).to_owned();
    assert_eq!(
        query.build(PostgresQueryBuilder).unwrap().0,
        "SELECT $1::\"ty\"\"pe\""
    );
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        "SELECT 'one'::\"ty\"\"pe\""
    );
}

/// Every name of up to three characters drawn from those that end quotes, strings, comments and
/// statements stays one identifier on every backend: the backend's own lexical rules read back
/// exactly the table and column it was given, and nothing after it.
#[test]
fn every_short_name_stays_one_identifier() {
    use crate::lexer::{Dialect, Kind, tokens};

    const CHARS: [char; 12] = [
        'a', '"', '`', '\'', '\\', '.', ' ', '\n', ';', '-', '*', 'ü',
    ];
    let mut names = vec![String::new()];
    let mut last = vec![String::new()];
    for _ in 0..3 {
        last = last
            .iter()
            .flat_map(|name| CHARS.map(|c| format!("{name}{c}")))
            .collect();
        names.extend(last.iter().cloned());
    }
    let check = |sql: String, dialect: Dialect, name: &str| {
        let read: Vec<_> = tokens(&sql, dialect)
            .filter(|token| token.kind != Kind::Space)
            .collect();
        let shape: Vec<_> = read.iter().map(|token| token.kind).collect();
        assert_eq!(
            shape,
            [
                Kind::Word,
                Kind::Identifier,
                Kind::Symbol,
                Kind::Identifier,
                Kind::Word,
                Kind::Identifier
            ],
            "{sql}"
        );
        for index in [1, 3, 5] {
            assert!(read[index].terminated, "{sql}");
            assert_eq!(read[index].unquoted().as_deref(), Some(name), "{sql}");
        }
    };
    for name in &names {
        let query = Query::select()
            .column((Alias::new(name.as_str()), Alias::new(name.as_str())))
            .from(Alias::new(name.as_str()))
            .to_owned();
        check(
            query.to_string(PostgresQueryBuilder).unwrap(),
            Dialect::Postgres,
            name,
        );
        check(
            query.to_string(MysqlQueryBuilder).unwrap(),
            Dialect::MySql,
            name,
        );
        check(
            query.to_string(SqliteQueryBuilder).unwrap(),
            Dialect::Sqlite,
            name,
        );
    }
}
