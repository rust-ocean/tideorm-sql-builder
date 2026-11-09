use super::*;

#[test]
fn inject_parameters_1() {
    assert_eq!(
        inject_parameters("WHERE A = ?", &["B".into()], &MysqlQueryBuilder).unwrap(),
        "WHERE A = 'B'"
    );
}

#[test]
fn inject_parameters_2() {
    assert_eq!(
        inject_parameters("WHERE A = '?' AND B = ?", &["C".into()], &MysqlQueryBuilder).unwrap(),
        "WHERE A = '?' AND B = 'C'"
    );
}

#[test]
fn inject_parameters_3() {
    assert_eq!(
        inject_parameters(
            "WHERE A = ? AND C = ?",
            &["B".into(), "D".into()],
            &MysqlQueryBuilder
        )
        .unwrap(),
        "WHERE A = 'B' AND C = 'D'"
    );
}

#[test]
fn inject_parameters_4() {
    assert_eq!(
        inject_parameters("?", &[vec![0xABu8, 0xCD, 0xEF].into()], &MysqlQueryBuilder).unwrap(),
        "x'ABCDEF'"
    );
}

#[test]
fn markers_follow_each_backends_lexical_rules() {
    // MySQL: a backslash escapes a quote in a string, `#` opens a comment,
    // and a backtick name holds no marker.
    assert_eq!(
        inject_parameters(r"'a\'?' `?` = ? # ?", &[1.into()], &MysqlQueryBuilder).unwrap(),
        r"'a\'?' `?` = 1 # ?"
    );
    // SQLite: no backslash escape, and `[..]` quotes a name.
    assert_eq!(
        inject_parameters(r"'a\' = ? AND [?] = 1", &[1.into()], &SqliteQueryBuilder).unwrap(),
        r"'a\' = 1 AND [?] = 1"
    );
    assert_eq!(
        inject_parameters("? /* ? */", &[], &MysqlQueryBuilder),
        Err(RenderError::MissingValue { index: 1, count: 0 })
    );
    assert_eq!(
        Expr::cust_with_values("? ?? ?", [1, 2], &MysqlQueryBuilder).map(|expr| Query::select()
            .expr(expr)
            .to_string(MysqlQueryBuilder)
            .unwrap()),
        Ok("SELECT 1 ? 2".to_owned())
    );
}
