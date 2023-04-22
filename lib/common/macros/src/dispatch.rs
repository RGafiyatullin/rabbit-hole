#[macro_export]
macro_rules! specialize_call {
    (@single_mapping,
        ( $($acc_p:pat),* ),
        ( $($acc_t:ty),* ),
        $func:tt, $args:tt, $select:tt,
        [ ($p:pat => $t:ty) $(, $p_t:tt)* $(,)* ],
        $mappings:tt
    ) => {
        specialize_call!(@specialize_call,
            ( $($acc_p,)* $p ),
            ( $($acc_t,)* $t ),

            $func, $args, $select,
            $mappings
        );

        specialize_call!(@single_mapping,
            ( $($acc_p),* ),
            ( $($acc_t),* ),
            $func, $args, $select,
            [ $($p_t),* ],
            $mappings
        );
    };
    (@single_mapping,
        $acc_p:tt, $acc_t: tt,
        $func:tt, $args:tt, $select:tt,
        [],
        $mappings:tt
    ) => {};

    (@specialize_call,
        $acc_p:tt,
        $acc_t:tt,
        $func:tt,
        $args:tt,
        $select:tt,

        ($head:tt $(, $tail:tt)*)
    ) => {
        specialize_call!(@single_mapping,
            $acc_p, $acc_t,
            $func, $args, $select,
            $head,
            ( $($tail),* )
        );
    };
    (@specialize_call,
        $acc_p:tt,
        $acc_t:tt,
        $func:tt,
        $args:tt,
        $select:tt,

        ()
    ) => {
        specialize_call!(@maybe_invoke,
            $acc_p,
            $acc_t,
            $select,
            $func,
            $args
        );
    };

    (@maybe_invoke,
        ( $($acc_p:pat),* ),
        ( $($acc_t:ty),* ),
        $select:expr,
        $func:ident,
        ($($arg:expr),*)
    ) => {
        #[allow(unused_parens)]
        if matches!($select, ( $($acc_p),* )) { break Some($func::<$($acc_t),*>( $($arg),* )); }
    };

    ($func:ident, ($($arg:expr),* $(,)*), $select:expr, $($mapping:tt),+ $(,)*) => {
        loop {
            specialize_call!(@specialize_call,
                (), (),
                $func,
                ($($arg),*),
                $select,
                ( $($mapping),* )
            );
            break None
        }
    };
}

#[cfg(test)]
mod tests;
