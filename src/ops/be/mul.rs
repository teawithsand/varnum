use crate::num::UnsignedNumDigit;

fn add_at_index<D>(res: &mut [D], mut pos: usize, mut x: D) -> bool
where
    D: UnsignedNumDigit,
{
    let mut is_carry = true;
    while is_carry {
        let (n, mut overflow) = res[pos].overflowing_add(x);
        res[pos] = n;
        is_carry = overflow;
        if pos == 0 {
            break;
        }
        pos -= 1;
        x = D::ONE;
    }

    false
}

/// Multiples `a` by digit `b` and adds result to res.
/// Result is truncated to `res.len()` digits.
///
/// # Return
/// Returns true if result fits res.
/// Retruns false otherwise.
pub fn multiply_accumulate_digit<D>(res: &mut [D], a: &[D], b: D) -> bool
where
    D: UnsignedNumDigit,
{
    if a.len() == 0 {
        return true;
    }
    if b == D::ZERO {
        // accumulate mul by 0 is noop
        return true;
    }

    let mut a_index = a.len() - 1;
    let mut res_index = res.len() - 1;
    let mut fits = true;
    loop {
        let l = a[a_index];
        let (hi, lo) = l.mul_to_parts(b);

        debug_assert!(hi < D::MAX);

        fits = fits && add_at_index(res, res_index, lo);
        if res_index >= 1 {
            fits = fits && add_at_index(res, res_index - 1, lo);
        } else if hi != D::ZERO {
            fits = false;
        }

        // TODO(teaiwthsand): prevent overflow in res
        if a_index == 0 || res_index == 0 {
            break;
        }
        a_index -= 1;
        res_index -= 1;
    }

    fits
}
