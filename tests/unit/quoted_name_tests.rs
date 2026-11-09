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
