#[macro_export]
macro_rules! pattern_enum {
    (
        $(#[$outer:meta])*
        $visibility:vis
        enum $name:ident {
            $(
                $(#[$inner:meta])*
                $variant:ident = $pattern:expr,
            )*
        }
    ) => {
        // ensure the patterns are decreasingly specific so that the first match is the correct one
        const _: () = {
            let patterns = [$($pattern),*];
            let mut i = 0;
            while i < patterns.len() - 1 {
                i += 1;
                let mut j = i + 1;
                while j < patterns.len() {
                    assert!(!const_str::starts_with!(patterns[j], patterns[i]), "patterns must be decreasingly specific");
                    j += 1;
                }
            }
        };

        $(#[$outer])*
        #[derive(PartialEq, Eq, Hash, Clone, Copy)]
        $visibility enum $name {
            $(
                $(#[$inner])*
                $variant,
            )*
        }

        impl From<&str> for $name {
            fn from(input: &str) -> Self {
                match input {
                    $(
                        $pattern => $name::$variant,
                    )*
                    _ => panic!("None of [{patterns}] match '{input}'", patterns = stringify!($($pattern),*))
                }
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        $name::$variant => write!(f, "{}", $pattern),
                    )*
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        $name::$variant => write!(f, "{}", $pattern),
                    )*
                }
            }
        }

        paste::paste! {
            #[derive(std::fmt::Debug, PartialEq, Eq, Hash, Clone, Copy)]
            enum [< $name Split >]<'a> {
                Pat($name),
                Str(&'a str),
            }
        }

        impl $name {
            pub fn split_once_and_match<'a>(input: &'a str) -> Option<(&'a str, Self, &'a str)> {
                $(
                    if let Some((prefix, suffix)) = input.split_once($pattern) {
                        return Some((prefix, $name::$variant, suffix));
                    }
                )*
                None
            }

            paste::paste! {
                pub fn split_match<'a>(input: &'a str) -> Vec<[< $name Split >]<'a>> {
                    let mut result = Vec::new();
                    let mut input = input;
                    while let Some((prefix, variant, suffix)) = Self::split_once_and_match(input) {
                        result.push([< $name Split >]::Str(prefix));
                        result.push([< $name Split >]::Pat(variant));
                        input = suffix;
                    }
                    result.push([< $name Split >]::Str(input));
                    result
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    pattern_enum! {
        enum Comparator {
            LTE = "<=",
            GTE = ">=",
            LT = "<",
            GT = ">",
            EQ = "=",
        }
    }

    pattern_enum! {
        enum Ops {
            LTE = "<=",
            GTE = ">=",
            LT = "<",
            GT = ">",
            EQ = "==",
            NEQ = "!=",
            ASSIGN = "=",
        }
    }

    #[test]
    fn from() {
        assert_eq!(Comparator::LT, Comparator::from("<"));
        assert_eq!(Comparator::GT, Comparator::from(">"));
        assert_eq!(Comparator::EQ, Comparator::from("="));
        assert_eq!(Comparator::LTE, Comparator::from("<="));
        assert_eq!(Comparator::GTE, Comparator::from(">="));
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Comparator::LT), "<");
        assert_eq!(format!("{:?}", Comparator::GT), ">");
        assert_eq!(format!("{:?}", Comparator::EQ), "=");
        assert_eq!(format!("{:?}", Comparator::LTE), "<=");
        assert_eq!(format!("{:?}", Comparator::GTE), ">=");
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Comparator::LT), "<");
        assert_eq!(format!("{}", Comparator::GT), ">");
        assert_eq!(format!("{}", Comparator::EQ), "=");
        assert_eq!(format!("{}", Comparator::LTE), "<=");
        assert_eq!(format!("{}", Comparator::GTE), ">=");
    }

    #[test]
    fn split_once_and_match() {
        assert_eq!(
            Comparator::split_once_and_match("x <= 5"),
            Some(("x ", Comparator::LTE, " 5"))
        );

        assert_eq!(
            Comparator::split_once_and_match("x >= 5"),
            Some(("x ", Comparator::GTE, " 5"))
        );

        assert_eq!(
            Comparator::split_once_and_match("x < 5"),
            Some(("x ", Comparator::LT, " 5"))
        );

        assert_eq!(
            Comparator::split_once_and_match("z<z<z"),
            Some(("z", Comparator::LT, "z<z"))
        );

        assert_eq!(
            Ops::split_once_and_match("x = true != false"),
            Some(("x ", Ops::ASSIGN, " true != false"))
        );
    }

    // #[test]
    // fn split_match() {
    //     assert_eq!(
    //         Comparator::split_match("x <= 5"),
    //         vec![
    //             ComparatorSplit::Str("x "),
    //             ComparatorSplit::Pat(Comparator::LTE),
    //             ComparatorSplit::Str(" 5"),
    //         ]
    //     );

    //     assert_eq!(
    //         Ops::split_match("x = true != false"),
    //         vec![
    //             OpsSplit::Str("x "),
    //             OpsSplit::Pat(Ops::ASSIGN),
    //             OpsSplit::Str(" true "),
    //             OpsSplit::Pat(Ops::NEQ),
    //             OpsSplit::Str(" false"),
    //         ]
    //     );
    // }
}
