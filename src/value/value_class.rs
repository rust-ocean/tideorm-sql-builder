//! Marker traits sorting value types into classes: numbers, dates, times and timestamps.
//! Each class has a `...Nullable` companion that also takes `Option` of a member.

use super::prelude::{
    DateTime, Decimal, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc,
};

/// Declares a class, its nullable companion (implemented for every member and for
/// `Option` of every member) and the members.
macro_rules! value_class {
    ($(#[$meta:meta])* $class:ident, $nullable:ident: $($member:ty),+ $(,)?) => {
        $(#[$meta])*
        pub trait $class: Sized {}

        #[doc = concat!("A [`", stringify!($class), "`] type, or `Option` of one.")]
        pub trait $nullable: Sized {}

        impl<T: $class> $nullable for T {}

        $(
            impl $class for $member {}
            impl $nullable for Option<$member> {}
        )+
    };
}

value_class!(
    /// Numbers: the integer types, both float types and `Decimal`.
    NumericValue, NumericValueNullable:
    i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, Decimal,
);

value_class!(
    /// Calendar dates.
    DateLikeValue, DateLikeValueNullable: NaiveDate,
);

value_class!(
    /// Times of day.
    TimeLikeValue, TimeLikeValueNullable: NaiveTime,
);

value_class!(
    /// Dates with a time of day, with or without an offset from UTC.
    DateTimeLikeValue, DateTimeLikeValueNullable:
    NaiveDateTime, DateTime<Utc>, DateTime<Local>, DateTime<FixedOffset>,
);
