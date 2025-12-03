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
pub fn sum_invalid_ids_a(interval: Interval) -> u64 {
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

// invalid ids are products of 11, 111, 1111,... 101, 10101, 10101... 1001, 1001001, 1001001001,... that don't have leading zeros
pub fn sum_invalid_ids_b(interval: Interval) -> u64 {
    let mut sum = 0;

    let start = *interval.start();
    let end = *interval.end();

    // caching the ids we've seen just as fast using a vec than a hashset or btreeset,
    // probably because each individual interval doesn't have too many ids in it
    let mut ids = Vec::new();

    let mut og_bases = (1..).map(|p| 10u64.pow(p) + 1);

    while let Some(og_base) = og_bases.next()
        && og_base < end
    {
        let mut base = og_base;
        // to get from an og base (11, 101, 1010) to the next one in its family,
        // multiply by (10, 100, 1000) and add 1
        // eg. 11 -> 111 -> 1111
        //     101 -> 10101 -> 1010101

        while base < end {
            // to avoid leading zeros, computer the smallest and largest legal mults for this base
            let lower_threshold = (og_base - 1) / 10;
            let upper_threshold = og_base - 2;

            // find the smallest mult greater or equal to than the lower bound,
            // and the largets mult less than or equal to the upper bound
            let smallest_mult = start.div_ceil(base).max(lower_threshold);
            let largest_mult = end.div_euclid(base).min(upper_threshold);

            if smallest_mult > largest_mult
                || smallest_mult * base < start
                || largest_mult * base > end
            {
                base = base * (og_base - 1) + 1;
                continue;
            }

            // go mult by mult and check if we've seen them already,
            // and if not, add them to the sum
            for mult in smallest_mult..=largest_mult {
                let id = mult * base;

                if !ids.contains(&id) {
                    sum += id;
                    ids.push(id);
                }
            }

            base = base * (og_base - 1) + 1;
        }
    }

    sum
}
