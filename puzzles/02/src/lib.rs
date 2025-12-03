use std::ops::RangeInclusive;

pub mod puzzle02a;
pub mod puzzle02b;

type Interval = RangeInclusive<u64>;

pub fn parse(input: &str) -> Vec<Interval> {
    input
        .trim()
        .split(',')
        .map(|seg| {
            let (left, right) = seg.split_once('-').unwrap();
            left.parse().unwrap()..=right.parse().unwrap()
        })
        .collect()
}

// invalid ids are products of 11, 101, 1001, that don't have leading zeros
pub fn sum_invalid_ids(interval: Interval) -> u64 {
    let mut sum = 0;

    // 11, 101, 1001, ...
    let mut bases = (1..).map(|p| 10u64.pow(p) + 1);

    let start = *interval.start();
    let end = *interval.end();

    while let Some(base) = bases.next()
        && base < end
    {
        // to avoid leading zeros, computer the smallest and largest legal mults for this base
        let lower_threshold = (base - 1) / 10;
        let upper_threshold = base - 2;

        // count the number of mults of base within the interval
        //
        // to do this, find the smallest mult greater or equal to than the lower bound,
        // and the largets mult less than or equal to the upper bound
        let smallest_mult = start.div_ceil(base).max(lower_threshold);
        let largest_mult = end.div_euclid(base).min(upper_threshold);

        if smallest_mult > largest_mult || smallest_mult * base < start || largest_mult * base > end
        {
            continue;
        }

        // compute the sum of all those multiples of `base`:
        // sum the integers between smallest_mult and largest_mult, then multiply the result by base
        let sum_of_mults = (largest_mult * (largest_mult + 1) / 2
            - smallest_mult * (smallest_mult - 1) / 2)
            * base;

        sum += sum_of_mults;
    }

    sum
}
