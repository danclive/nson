// nson macro based on the serde_json json! implementation.

/// Construct a value::Value value from a literal.
///
/// ```rust
/// # #[macro_use]
/// # extern crate nson;
/// #
/// # fn main() {
/// let value = nson!({
///     "code": 200,
///     "success": true,
///     "payload": {
///         "some": [
///             "pay",
///             "loads",
///         ]
///     }
/// });
/// # }
/// ```
#[macro_export]
macro_rules! nson {
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: nson!(@array [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Finished with trailing comma.
    (@array [$($elems:expr,)*]) => {
        $crate::core::array::Array::from_vec(vec![$($elems,)*])
    };

    // Finished without trailing comma.
    (@array [$($elems:expr),*]) => {
        $crate::core::array::Array::from_vec(vec![$($elems,)*])
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        $crate::core::nson!(@array [$($elems,)* $crate::core::nson!(null)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        $crate::core::nson!(@array [$($elems,)* $crate::core::nson!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        $crate::core::nson!(@array [$($elems,)* $crate::core::nson!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        $crate::core::nson!(@array [$($elems,)* $crate::core::nson!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        $crate::core::nson!(@array [$($elems,)* $crate::core::nson!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        $crate::core::nson!(@array [$($elems,)*] $($rest)*)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: nson!(@object $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Finished.
    (@object $object:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        $object.insert_value(($($key)+), $value);
        $crate::core::nson!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        $object.insert_value(($($key)+), $value);
    };

    // Next value is `null`.
    (@object $object:ident ($($key:tt)+) (=> null $($rest:tt)*) $copy:tt) => {
        $crate::core::nson!(@object $object [$($key)+] ($crate::core::nson!(null)) $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        $crate::core::nson!(@object $object [$($key)+] ($crate::core::nson!(null)) $($rest)*);
    };

    // Next value is an array.
    (@object $object:ident ($($key:tt)+) (=> [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        $crate::core::nson!(@object $object [$($key)+] ($crate::core::nson!([$($array)*])) $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        $crate::core::nson!(@object $object [$($key)+] ($crate::core::nson!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@object $object:ident ($($key:tt)+) (=> {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        $crate::core::nson!(@object $object [$($key)+] ($crate::core::nson!({$($map)*})) $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        $crate::core::nson!(@object $object [$($key)+] ($crate::core::nson!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (=> $value:expr , $($rest:tt)*) $copy:tt) => {
        $crate::core::nson!(@object $object [$($key)+] ($crate::core::nson!($value)) , $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        $crate::core::nson!(@object $object [$($key)+] ($crate::core::nson!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (=> $value:expr) $copy:tt) => {
        $crate::core::nson!(@object $object [$($key)+] ($crate::core::nson!($value)));
    };

    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        $crate::core::nson!(@object $object [$($key)+] ($crate::core::nson!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) (=>) $copy:tt) => {
        // "unexpected end of macro invocation"
        $crate::core::nson!();
    };

    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        $crate::core::nson!();
    };

    // Missing key-value separator and value for last entry.
    // Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        $crate::core::nson!();
    };

    // Misplaced key-value separator. Trigger a reasonable error message.
    (@object $object:ident () (=> $($rest:tt)*) ($kv_separator:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `=>`".
        unimplemented!($kv_separator);
    };

    (@object $object:ident () (: $($rest:tt)*) ($kv_separator:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        unimplemented!($kv_separator);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        unimplemented!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) => $($rest:tt)*) $copy:tt) => {
        $crate::core::nson!(@object $object ($key) (=> $($rest)*) (=> $($rest)*));
    };

    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        $crate::core::nson!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        $crate::core::nson!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: nson!($($nson)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        $crate::core::value::Value::Null
    };

    ([]) => {
        $crate::core::value::Value::Array(vec![].into())
    };

    ([ $($tt:tt)+ ]) => {
        $crate::core::value::Value::Array($crate::core::nson!(@array [] $($tt)+))
    };

    ({}) => {
        $crate::core::value::Value::Message($crate::core::msg!{})
    };

    ({$($tt:tt)+}) => {
        $crate::core::value::Value::Message($crate::core::msg!{$($tt)+});
    };

    // Any Serialize type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:expr) => {
        std::convert::From::from($other)
    };
}

/// Construct a msg::Message value.
///
/// ```rust
/// # #[macro_use]
/// # extern crate nson;
/// #
/// # fn main() {
/// let value = msg! {
///     "code": 200,
///     "success": true,
///     "payload": {
///         "some": [
///             "pay",
///             "loads",
///         ]
///     }
/// };
/// # }
/// ```
#[macro_export]
macro_rules! msg {
    () => {{ $crate::core::message::Message::with_capacity(8) }};
    ( $($tt:tt)+ ) => {{
        let mut object = $crate::core::message::Message::with_capacity(8);
        $crate::core::nson!(@object object () ($($tt)+) ($($tt)+));
        object
    }};
}
