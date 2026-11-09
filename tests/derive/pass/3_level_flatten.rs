use sql_builder::Iden;

#[derive(Copy, Clone, Iden)]
enum Asset {
    Table,
    Id,
    AssetName,
    #[iden(flatten)]
    First {
        first: FirstLevel,
    },
}

#[derive(Copy, Clone, Iden)]
enum FirstLevel {
    LevelOne,
    #[iden(flatten)]
    Second(SecondLevel),
}

#[derive(Copy, Clone, Iden)]
enum SecondLevel {
    LevelTwo,
    #[iden(flatten)]
    Third(LevelThree),
    UserId,
}

#[derive(Copy, Clone, Iden, Default)]
struct LevelThree;

impl Default for FirstLevel {
    fn default() -> Self {
        Self::LevelOne
    }
}

fn main() {
    // custom ends up being default string which is an empty string
    let expected = [
        "asset",
        "id",
        "asset_name",
        "level_one",
        "level_two",
        "level_three",
        "user_id",
    ];
    [
        Asset::Table,
        Asset::Id,
        Asset::AssetName,
        Asset::First {
            first: FirstLevel::default(),
        },
    ]
    .into_iter()
    .chain(
        [
            SecondLevel::LevelTwo,
            SecondLevel::Third(LevelThree),
            SecondLevel::UserId,
        ]
        .into_iter()
        .map(FirstLevel::Second)
        .map(|s| Asset::First { first: s }),
    )
    .zip(expected)
    .for_each(|(var, exp)| {
        assert_eq!(var.to_string(), exp);
    })
}
