#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(test)]
mod tests {
    use bytecheck::CheckBytes;

    fn check_as_bytes<T: CheckBytes<C>, C>(value: &T, mut context: C) {
        unsafe { T::check_bytes(value as *const T, &mut context).unwrap() };
    }

    #[test]
    fn test_tuples() {
        check_as_bytes(&(42u32, true, 'x'), &mut ());
        check_as_bytes(&(true,), &mut ());

        unsafe {
            // These tests assume the tuple is packed (u32, bool, char)
            <(u32, bool, char)>::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 1u8, 255u8, 255u8, 255u8, 0x78u8, 0u8, 0u8, 0u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap();
            <(u32, bool, char)>::check_bytes(
                (&[
                    42u8, 16u8, 20u8, 3u8, 1u8, 255u8, 255u8, 255u8, 0x78u8, 0u8, 0u8, 0u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap();
            <(u32, bool, char)>::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 1u8, 255u8, 255u8, 255u8, 0x00u8, 0xd8u8, 0u8, 0u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap_err();
            <(u32, bool, char)>::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 1u8, 255u8, 255u8, 255u8, 0x00u8, 0x00u8, 0x11u8, 0u8,
                    255u8, 255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap_err();
            <(u32, bool, char)>::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 255u8, 255u8, 0x78u8, 0u8, 0u8, 0u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap();
            <(u32, bool, char)>::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 2u8, 255u8, 255u8, 255u8, 0x78u8, 0u8, 0u8, 0u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap_err();
        }
    }

    #[test]
    fn test_arrays() {
        check_as_bytes(&[true, false, true, false], &mut ());
        check_as_bytes(&[false, true], &mut ());

        unsafe {
            <[bool; 4]>::check_bytes((&[1u8, 0u8, 1u8, 0u8] as *const u8).cast(), &mut ()).unwrap();
            <[bool; 4]>::check_bytes((&[1u8, 2u8, 1u8, 0u8] as *const u8).cast(), &mut ())
                .unwrap_err();
            <[bool; 4]>::check_bytes((&[2u8, 0u8, 1u8, 0u8] as *const u8).cast(), &mut ())
                .unwrap_err();
            <[bool; 4]>::check_bytes((&[1u8, 0u8, 1u8, 2u8] as *const u8).cast(), &mut ())
                .unwrap_err();
            <[bool; 4]>::check_bytes((&[1u8, 0u8, 1u8, 0u8, 2u8] as *const u8).cast(), &mut ())
                .unwrap();
        }
    }

    #[test]
    fn test_unit_struct() {
        #[derive(CheckBytes)]
        struct Test;

        check_as_bytes(&Test, &mut ());
    }

    #[test]
    fn test_tuple_struct() {
        #[derive(CheckBytes, Debug)]
        struct Test(u32, bool, char);

        let value = Test(42, true, 'x');

        check_as_bytes(&value, &mut ());

        unsafe {
            // These tests assume the struct is packed (u32, char, bool)
            Test::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 0x78u8, 0u8, 0u8, 0u8, 1u8, 255u8, 255u8, 255u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap();
            Test::check_bytes(
                (&[
                    42u8, 16u8, 20u8, 3u8, 0x78u8, 0u8, 0u8, 0u8, 1u8, 255u8, 255u8, 255u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap();
            Test::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 0x00u8, 0xd8u8, 0u8, 0u8, 1u8, 255u8, 255u8, 255u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap_err();
            Test::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 0x00u8, 0x00u8, 0x11u8, 0u8, 1u8, 255u8, 255u8, 255u8,
                    255u8, 255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap_err();
            Test::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 0x78u8, 0u8, 0u8, 0u8, 0u8, 255u8, 255u8, 255u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap();
            Test::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 0x78u8, 0u8, 0u8, 0u8, 2u8, 255u8, 255u8, 255u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap_err();
        }
    }

    #[test]
    fn test_struct() {
        #[derive(CheckBytes, Debug)]
        struct Test {
            a: u32,
            b: bool,
            c: char,
        }

        let value = Test {
            a: 42,
            b: true,
            c: 'x',
        };

        check_as_bytes(&value, &mut ());

        unsafe {
            // These tests assume the struct is packed (u32, char, bool)
            Test::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 0x78u8, 0u8, 0u8, 0u8, 1u8, 255u8, 255u8, 255u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap();
            Test::check_bytes(
                (&[
                    42u8, 16u8, 20u8, 3u8, 0x78u8, 0u8, 0u8, 0u8, 1u8, 255u8, 255u8, 255u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap();
            Test::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 0x00u8, 0xd8u8, 0u8, 0u8, 1u8, 255u8, 255u8, 255u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap_err();
            Test::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 0x00u8, 0x00u8, 0x11u8, 0u8, 1u8, 255u8, 255u8, 255u8,
                    255u8, 255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap_err();
            Test::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 0x78u8, 0u8, 0u8, 0u8, 0u8, 255u8, 255u8, 255u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap();
            Test::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 0x78u8, 0u8, 0u8, 0u8, 2u8, 255u8, 255u8, 255u8, 255u8,
                    255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap_err();
        }
    }

    #[test]
    fn test_generic_struct() {
        #[derive(CheckBytes, Debug)]
        struct Test<T> {
            a: u32,
            b: T,
        }

        let value = Test { a: 42, b: true };

        check_as_bytes(&value, &mut ());

        unsafe {
            Test::<bool>::check_bytes(
                (&[0u8, 0u8, 0u8, 0u8, 1u8, 255u8, 255u8, 255u8] as *const u8).cast(),
                &mut (),
            )
            .unwrap();
            Test::<bool>::check_bytes(
                (&[12u8, 34u8, 56u8, 78u8, 1u8, 255u8, 255u8, 255u8] as *const u8).cast(),
                &mut (),
            )
            .unwrap();
            Test::<bool>::check_bytes(
                (&[0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 255u8, 255u8] as *const u8).cast(),
                &mut (),
            )
            .unwrap();
            Test::<bool>::check_bytes(
                (&[0u8, 0u8, 0u8, 0u8, 2u8, 255u8, 255u8, 255u8] as *const u8).cast(),
                &mut (),
            )
            .unwrap_err();
        }
    }

    #[test]
    fn test_enum() {
        #[derive(CheckBytes, Debug)]
        #[repr(u8)]
        enum Test {
            A(u32, bool, char),
            #[allow(dead_code)]
            B {
                a: u32,
                b: bool,
                c: char,
            },
            C,
        }

        let value = Test::A(42, true, 'x');

        check_as_bytes(&value, &mut ());

        let value = Test::B {
            a: 42,
            b: true,
            c: 'x',
        };

        check_as_bytes(&value, &mut ());

        let value = Test::C;

        check_as_bytes(&value, &mut ());

        unsafe {
            Test::check_bytes(
                (&[
                    0u8, 0u8, 0u8, 0u8, 12u8, 34u8, 56u8, 78u8, 1u8, 255u8, 255u8, 255u8, 120u8,
                    0u8, 0u8, 0u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap();
            Test::check_bytes(
                (&[
                    1u8, 0u8, 0u8, 0u8, 12u8, 34u8, 56u8, 78u8, 1u8, 255u8, 255u8, 255u8, 120u8,
                    0u8, 0u8, 0u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap();
            Test::check_bytes(
                (&[
                    2u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                    255u8, 25u8, 255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap();
            Test::check_bytes(
                (&[
                    3u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                    255u8, 25u8, 255u8, 255u8, 255u8,
                ] as *const u8)
                    .cast(),
                &mut (),
            )
            .unwrap_err();
        }
    }

    #[test]
    fn test_explicit_enum_values() {
        #[derive(CheckBytes, Debug)]
        #[repr(u8)]
        enum Test {
            A,
            B = 100,
            C,
            D = 200,
            E,
        }

        check_as_bytes(&Test::A, &mut ());
        check_as_bytes(&Test::B, &mut ());
        check_as_bytes(&Test::C, &mut ());
        check_as_bytes(&Test::D, &mut ());
        check_as_bytes(&Test::E, &mut ());

        unsafe {
            Test::check_bytes((&[1u8] as *const u8).cast(), &mut ()).unwrap_err();
            Test::check_bytes((&[99u8] as *const u8).cast(), &mut ()).unwrap_err();
            Test::check_bytes((&[102u8] as *const u8).cast(), &mut ()).unwrap_err();
            Test::check_bytes((&[199u8] as *const u8).cast(), &mut ()).unwrap_err();
            Test::check_bytes((&[202u8] as *const u8).cast(), &mut ()).unwrap_err();
            Test::check_bytes((&[255u8] as *const u8).cast(), &mut ()).unwrap_err();
        }
    }

    #[test]
    fn test_context() {
        use core::{convert::Infallible, fmt};

        #[derive(Debug)]
        #[repr(transparent)]
        struct Test(i32);

        struct TestContext(pub i32);

        #[derive(Debug)]
        struct TestError {
            expected: i32,
            found: i32,
        }

        impl fmt::Display for TestError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "mismatched value: expected {}, found {}",
                    self.expected, self.found
                )
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for TestError {}

        impl From<Infallible> for TestError {
            fn from(_: Infallible) -> Self {
                unsafe { core::hint::unreachable_unchecked() }
            }
        }

        impl CheckBytes<TestContext> for Test {
            type Error = TestError;

            unsafe fn check_bytes<'a>(
                value: *const Test,
                context: &mut TestContext,
            ) -> Result<&'a Self, Self::Error> {
                let found = *i32::check_bytes(value.cast(), context)?;
                let expected = context.0;
                if expected == found {
                    Ok(&*value)
                } else {
                    Err(TestError { expected, found })
                }
            }
        }

        unsafe {
            Test::check_bytes(
                (&[42u8, 0u8, 0u8, 0u8] as *const u8).cast(),
                &mut TestContext(42),
            )
            .unwrap();
            Test::check_bytes(
                (&[41u8, 0u8, 0u8, 0u8] as *const u8).cast(),
                &mut TestContext(42),
            )
            .unwrap_err();
        }

        #[repr(transparent)]
        #[derive(CheckBytes, Debug)]
        struct TestContainer(Test);

        unsafe {
            TestContainer::check_bytes(
                (&[42u8, 0u8, 0u8, 0u8] as *const u8).cast(),
                &mut TestContext(42),
            )
            .unwrap();
            TestContainer::check_bytes(
                (&[41u8, 0u8, 0u8, 0u8] as *const u8).cast(),
                &mut TestContext(42),
            )
            .unwrap_err();
        }
    }

    #[test]
    fn test_unsized() {
        unsafe {
            <[i32] as CheckBytes<()>>::check_bytes(
                &[1, 2, 3, 4] as &[i32] as *const [i32],
                &mut (),
            )
            .unwrap();
            <str as CheckBytes<()>>::check_bytes("hello world" as *const str, &mut ()).unwrap();
        }
    }

    #[test]
    fn test_recursive() {
        struct MyBox<T: ?Sized> {
            inner: *const T,
        }

        impl<T: CheckBytes<C>, C: Default + ?Sized> CheckBytes<C> for MyBox<T> {
            type Error = T::Error;

            unsafe fn check_bytes<'a>(
                value: *const Self,
                context: &mut C,
            ) -> Result<&'a Self, Self::Error> {
                T::check_bytes((*value).inner, context)?;
                Ok(&*value)
            }
        }

        #[derive(CheckBytes)]
        #[check_bytes(bound = "__C: Default")]
        #[repr(u8)]
        enum Node {
            Nil,
            Cons(#[omit_bounds] MyBox<Node>),
        }

        unsafe {
            let nil = Node::Nil;
            let cons = Node::Cons(MyBox {
                inner: &nil as *const Node,
            });
            Node::check_bytes(&cons, &mut ()).unwrap();
        }
    }
}
