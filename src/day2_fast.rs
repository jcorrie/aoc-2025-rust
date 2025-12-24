
use crate::reuse;

/// safe pow10 for u128, returns 10^k
fn pow10(k: usize) -> u128 {
    let mut p: u128 = 1;
    for _ in 0..k {
        p = p.saturating_mul(10);
    }
    p
}

fn digits_count(mut n: u128) -> usize {
    if n == 0 { return 1; }
    let mut d = 0;
    while n > 0 {
        d += 1;
        n /= 10;
    }
    d
}

/// Sum all doubled numbers (m repeated twice -> m||m) inside [start, end].
/// A doubled number for half-length k is val = m * 10^k + m = m * (10^k + 1).
fn sum_doubled_in_range(start: u128, end: u128) -> u128 {
    if start > end { return 0; }
    let mut sum: u128 = 0;

    // Determine the allowed total digits range, and therefore half-length k range.
    let min_digits = digits_count(start);
    let max_digits = digits_count(end);

    // half-length k ranges from ceil(min_digits / 2) to floor(max_digits / 2)
    let k_min = (min_digits + 1) / 2;
    let k_max = max_digits / 2;

    for k in k_min..=k_max {
        let ten_k = pow10(k);
        // divisor is (10^k + 1) because val = m * (10^k + 1)
        let div = ten_k.checked_add(1).expect("overflow on div");

        // m must be k digits and not start with zero: m_lo_base..=m_hi_base
        let m_lo_base = pow10(k - 1);        // 10^(k-1), e.g. for k=1 -> 1
        let m_hi_base = ten_k - 1;           // 10^k - 1

        // compute m_low = ceil(start / div)
        let m_low = {
            let q = start / div;
            if q * div == start { q } else { q + 1 }
        };

        // compute m_high = floor(end / div)
        let m_high = end / div;

        // intersect with k-digit range
        let m_from = if m_lo_base > m_low { m_lo_base } else { m_low };
        let m_to = if m_hi_base < m_high { m_hi_base } else { m_high };

        if m_from > m_to {
            continue;
        }

        // iterate only the necessary m values
        for m in m_from..=m_to {
            // val = m * (10^k + 1)
            let val = m.saturating_mul(div);
            if val >= start && val <= end {
                sum = sum.saturating_add(val);
            }
        }
    }

    sum
}

pub fn main(input: &str) -> u128 {
    let ranges = reuse::split_string_to_list(input, ',');
    let mut total: u128 = 0;

    for r in ranges.iter() {
        let parts: Vec<&str> = reuse::split_string_to_list(r, '-');
        if parts.len() != 2 { continue; }
        let start: u128 = reuse::string_to_usize(parts[0].trim()) as u128;
        let end: u128 = reuse::string_to_usize(parts[1].trim()) as u128;
        total = total.saturating_add(sum_doubled_in_range(start, end));
    }

    total
}