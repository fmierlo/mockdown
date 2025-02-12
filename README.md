# Mockdown &emsp; [![latest]][crates.io] [![msrv]][crates.io] [![docs.rs]][docs.rs/mockdown] ![][build]

Mockdown is a single file and macro/dependency free mock library for Rust.

# Example

## Simple

```rust
pub mod math {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

pub mod plus {
    #[cfg(not(test))]
    use super::math;

    #[cfg(test)]
    use mocks::math;

    pub fn one(x: i32) -> i32 {
        math::add(x, 1)
    }

    #[cfg(test)]
    pub mod mocks {
        pub mod math {
            use mockdown::{mockdown, Mock};

            #[derive(Debug, PartialEq)]
            pub struct Add(pub i32, pub i32);

            pub fn add(x: i32, y: i32) -> i32 {
                let args = Add(x, y);
                mockdown().mock(args).unwrap()
            }
        }
    }

    #[cfg(test)]
    pub mod tests {
        use mockdown::{mockdown, Mock};
        use super::mocks::math;

        #[test]
        pub fn test_one() {
            mockdown().expect(|args| {
                assert_eq!(math::Add(1, 1), args);
                2
            });

            let z = super::one(1);
            assert_eq!(z, 2);
        }
    }
}
```

[latest]: https://img.shields.io/crates/v/mockdown.svg
[crates.io]: https://crates.io/crates/mockdown
[msrv]: https://img.shields.io/crates/msrv/mockdown
[docs.rs]: https://img.shields.io/docsrs/mockdown?label=docs.rs
[docs.rs/mockdown]: https://docs.rs/mockdown/
[build]: https://github.com/fmierlo/mockdown/workflows/test/badge.svg
