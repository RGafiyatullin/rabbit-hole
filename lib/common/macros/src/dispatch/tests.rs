struct A<const I: usize>();
struct B<const I: usize>();
struct C<const I: usize>();

#[derive(Debug)]
enum Select<const I: usize> {
    A,
    B,
    C,
}

use core::any::type_name as tn;

#[test]
fn specialize_call_1() {
    fn do_it<T>(_arg: usize) -> &'static str {
        tn::<T>()
    }

    assert_eq!(
        Some(tn::<A<1>>()),
        specialize_call!(do_it, (1), Select::<1>::A, [
                (Select::<1>::A => A::<1>),
                (Select::<1>::B => B::<1>),
            ])
    );

    assert_eq!(
        Some(tn::<B<1>>()),
        specialize_call!(do_it, (2), Select::<1>::B, [
                (Select::<1>::A => A::<1>),
                (Select::<1>::B => B::<1>),
            ])
    );

    assert_eq!(
        None::<&str>,
        specialize_call!(do_it, (3), Select::<1>::C, [
                (Select::<1>::A => A::<1>),
                (Select::<1>::B => B::<1>),
            ])
    );
}

#[test]
fn specialize_call_2() {
    fn do_it<T1, T2>(_arg: usize) -> (&'static str, &'static str) {
        (tn::<T1>(), tn::<T2>())
    }

    assert_eq!(
        Some((tn::<A<1>>(), tn::<C<2>>())),
        specialize_call!(do_it, (1), (Select::<1>::A, Select::<2>::C),
            [
                (Select::<1>::A => A::<1>),
                (Select::<1>::B => B::<1>),
            ], [
                (Select::<2>::B => B::<2>),
                (Select::<2>::C => C::<2>),
            ]
        )
    );

    assert_eq!(
        Some((tn::<B<1>>(), tn::<B<2>>())),
        specialize_call!(do_it, (1), (Select::<1>::B, Select::<2>::B),
            [
                (Select::<1>::A => A::<1>),
                (Select::<1>::B => B::<1>),
            ], [
                (Select::<2>::B => B::<2>),
                (Select::<2>::C => C::<2>),
            ]
        )
    );

    assert_eq!(
        None::<(&str, &str)>,
        specialize_call!(do_it, (1), (Select::<1>::C, Select::<2>::C),
            [
                (Select::<1>::A => A::<1>),
                (Select::<1>::B => B::<1>),
            ], [
                (Select::<2>::B => B::<2>),
                (Select::<2>::C => C::<2>),
            ]
        )
    );
    assert_eq!(
        None::<(&str, &str)>,
        specialize_call!(do_it, (1), (Select::<1>::A, Select::<2>::A),
            [
                (Select::<1>::A => A::<1>),
                (Select::<1>::B => B::<1>),
            ], [
                (Select::<2>::B => B::<2>),
                (Select::<2>::C => C::<2>),
            ]
        )
    );
}
