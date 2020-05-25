use std::cmp;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub struct Time {
    ms: u64,
}

impl Time {
    fn new(ms: u64) -> Self {
        Time { ms }
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ms.cmp(&other.ms)
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn in_hours(t: Time) -> u64 {
    const MS_PER_HOUR: u64 = 1000 * 60 * 60;
    t.ms / MS_PER_HOUR
}

fn greater(t1: Time, t2: Time) -> Time {
    cmp::max(t1, t2)
}

// implement without cloning
fn greatest(v: Vec<Time>) -> Time {
    let max_time = v.iter().max().unwrap();
    Time::new(max_time.ms)
}

// references
fn time_diff_in_ms(t1: &Time, t2: &Time) -> u64 {
    match t1.cmp(t2) {
        Ordering::Greater => t1.ms - t2.ms,
        Ordering::Equal => 0,
        Ordering::Less => t2.ms - t1.ms,
    }
}

fn greatest_ref(v: &Vec<Time>) -> &Time {
    v.iter().max().unwrap()
}

fn main() {}

mod test {
    use super::*;
    #[test]
    fn test_in_hours() {
        let t = Time {
            ms: 3600 * 1000 * 5,
        };
        assert_eq!(5, in_hours(t));
    }

    #[test]
    fn test_greater() {
        let t1 = Time::new(5);
        let t2 = Time::new(1);
        let res = greater(t1, t2);
        assert_eq!(5, res.ms);
    }

    #[test]
    fn test_greatest() {
        let v = vec![Time::new(1), Time::new(5)];
        let res = greatest(v);
        assert_eq!(5, res.ms);
    }

    #[test]
    fn test_time_diff() {
        let t1 = Time::new(1);
        let t2 = Time::new(10);
        assert_eq!(9, time_diff_in_ms(&t1, &t2));
        assert_eq!(9, time_diff_in_ms(&t2, &t1));
    }

    #[test]
    fn test_greatest_ref() {
        let v = vec![Time::new(1), Time::new(5)];
        let res = greatest_ref(&v);
        assert_eq!(5, res.ms);
    }
}
