use std::{fmt::Debug, fmt::Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RangeSet {
    from: u64,
    to: u64,
}

impl RangeSet {
    pub fn new(from: u64, to: u64) -> RangeSet {
        assert!(
            from <= to,
            "from must be less than or equal to to, but received [{from}, {to}]",
        );
        RangeSet { from, to }
    }

    pub fn contains(&self, value: u64) -> bool {
        value >= self.from && value <= self.to
    }

    pub fn intersection(&self, other: &RangeSet) -> Option<RangeSet> {
        if self.from > other.to || self.to < other.from {
            None
        } else {
            Some(RangeSet {
                from: self.from.max(other.from),
                to: self.to.min(other.to),
            })
        }
    }

    pub fn union(&self, other: &RangeSet) -> Option<RangeSet> {
        if self.from > other.to || self.to < other.from {
            None
        } else {
            Some(RangeSet {
                from: self.from.min(other.from),
                to: self.to.max(other.to),
            })
        }
    }

    pub fn len(&self) -> u64 {
        self.to - self.from + 1
    }

    pub fn iter(&self) -> impl Iterator<Item = u64> {
        (self.from..=self.to).into_iter()
    }

    pub fn subset_greater_than(&self, value: u64) -> Option<RangeSet> {
        if value >= self.to {
            None
        } else {
            Some(RangeSet {
                from: value + 1,
                to: self.to,
            })
        }
    }

    pub fn subset_less_than(&self, value: u64) -> Option<RangeSet> {
        if value <= self.from {
            None
        } else {
            Some(RangeSet {
                from: self.from,
                to: value - 1,
            })
        }
    }

    pub fn subset_greater_than_or_equal(&self, value: u64) -> Option<RangeSet> {
        if value > self.to {
            None
        } else {
            Some(RangeSet {
                from: value,
                to: self.to,
            })
        }
    }

    pub fn subset_less_than_or_equal(&self, value: u64) -> Option<RangeSet> {
        if value < self.from {
            None
        } else {
            Some(RangeSet {
                from: self.from,
                to: value,
            })
        }
    }

    pub fn partition_upper_inclusive(&self, value: u64) -> (Option<RangeSet>, Option<RangeSet>) {
        (
            self.subset_less_than(value),
            self.subset_greater_than_or_equal(value),
        )
    }

    pub fn partition_lower_inclusive(&self, value: u64) -> (Option<RangeSet>, Option<RangeSet>) {
        (
            self.subset_less_than_or_equal(value),
            self.subset_greater_than(value),
        )
    }
}

impl Debug for RangeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RangeSet[{}, {}]", self.from, self.to)
    }
}

impl Display for RangeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.from, self.to)
    }
}

impl From<(u64, u64)> for RangeSet {
    fn from((from, to): (u64, u64)) -> Self {
        RangeSet::new(from, to)
    }
}

impl From<RangeSet> for (u64, u64) {
    fn from(range_set: RangeSet) -> Self {
        (range_set.from, range_set.to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_set_intersection_works() {
        let a = RangeSet { from: 0, to: 10 };
        let b = RangeSet { from: 5, to: 15 };
        let c = RangeSet { from: 0, to: 5 };
        let d = RangeSet { from: 10, to: 15 };
        let e = RangeSet { from: 5, to: 10 };
        let f = RangeSet { from: 0, to: 15 };
        let g = RangeSet { from: 5, to: 5 };
        let h = RangeSet { from: 15, to: 15 };

        assert_eq!(a.intersection(&b), Some(RangeSet { from: 5, to: 10 }));
        assert_eq!(a.intersection(&c), Some(RangeSet { from: 0, to: 5 }));
        assert_eq!(a.intersection(&d), Some(RangeSet { from: 10, to: 10 }));
        assert_eq!(a.intersection(&e), Some(RangeSet { from: 5, to: 10 }));
        assert_eq!(a.intersection(&f), Some(RangeSet { from: 0, to: 10 }));
        assert_eq!(a.intersection(&g), Some(RangeSet { from: 5, to: 5 }));
        assert_eq!(a.intersection(&h), None);
    }

    #[test]
    fn range_set_union_works() {
        let a = RangeSet { from: 0, to: 10 };
        let b = RangeSet { from: 5, to: 15 };
        let c = RangeSet { from: 20, to: 30 };

        assert_eq!(a.union(&b), Some(RangeSet { from: 0, to: 15 }));
        assert_eq!(a.union(&c), None);
    }
}
