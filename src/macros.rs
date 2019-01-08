// NSON macro based on the serde_json json! implementation.

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
        $crate::value::Array::from_vec(vec![$($elems,)*])
    };

    // Finished without trailing comma.
    (@array [$($elems:expr),*]) => {
        $crate::value::Array::from_vec(vec![$($elems,)*])
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        nson!(@array [$($elems,)* nson!(null)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        nson!(@array [$($elems,)* nson!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        nson!(@array [$($elems,)* nson!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        nson!(@array [$($elems,)* nson!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        nson!(@array [$($elems,)* nson!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        nson!(@array [$($elems,)*] $($rest)*)
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
        $object.insert_value(($($key)+).into(), $value);
        nson!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        $object.insert_value(($($key)+).into(), $value);
    };

    // Next value is `null`.
    (@object $object:ident ($($key:tt)+) (=> null $($rest:tt)*) $copy:tt) => {
        nson!(@object $object [$($key)+] (nson!(null)) $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        nson!(@object $object [$($key)+] (nson!(null)) $($rest)*);
    };

    // Next value is an array.
    (@object $object:ident ($($key:tt)+) (=> [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        nson!(@object $object [$($key)+] (nson!([$($array)*])) $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        nson!(@object $object [$($key)+] (nson!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@object $object:ident ($($key:tt)+) (=> {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        nson!(@object $object [$($key)+] (nson!({$($map)*})) $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        nson!(@object $object [$($key)+] (nson!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (=> $value:expr , $($rest:tt)*) $copy:tt) => {
        nson!(@object $object [$($key)+] (nson!($value)) , $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        nson!(@object $object [$($key)+] (nson!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (=> $value:expr) $copy:tt) => {
        nson!(@object $object [$($key)+] (nson!($value)));
    };

    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        nson!(@object $object [$($key)+] (nson!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) (=>) $copy:tt) => {
        // "unexpected end of macro invocation"
        nson!();
    };

    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        nson!();
    };

    // Missing key-value separator and value for last entry.
    // Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        nson!();
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
        nson!(@object $object ($key) (=> $($rest)*) (=> $($rest)*));
    };

    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        nson!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        nson!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: nson!($($nson)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        $crate::value::Value::Null
    };

    ([]) => {
        $crate::value::Value::Array(vec![].into())
    };

    ([ $($tt:tt)+ ]) => {
        $crate::value::Value::Array(nson!(@array [] $($tt)+))
    };

    ({}) => {
        $crate::value::Value::Message(msg!{})
    };

    ({$($tt:tt)+}) => {
        $crate::value::Value::Message(msg!{$($tt)+});
    };

    // Any Serialize type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:expr) => {
        std::convert::From::from($other)
    };
}

/// Construct a nson::Message value.
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
    () => {{ $crate::message::Message::new() }};
    ( $($tt:tt)+ ) => {{
        let mut object = $crate::message::Message::new();
        nson!(@object object () ($($tt)+) ($($tt)+));
        object
    }};
}
