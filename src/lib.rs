/// Shipping is 500 cents for orders below 5000 cents.
pub fn total_with_shipping(prices: &[u64]) -> Result<u64, &'static str> {
    let subtotal = prices.iter().try_fold(0_u64, |sum, &price| sum.checked_add(price))
        .ok_or("price total is too large")?;
    let shipping = if subtotal >= 5000 { 0 } else { 500 };
    subtotal.checked_add(shipping).ok_or("price total is too large")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_threshold_has_free_shipping() {
        assert_eq!(total_with_shipping(&[2500, 2500]), Ok(5000));
    }

    #[test]
    fn below_threshold_adds_shipping() {
        assert_eq!(total_with_shipping(&[2000, 2000]), Ok(4500));
    }

    #[test]
    fn above_threshold_has_free_shipping() {
        assert_eq!(total_with_shipping(&[3000, 3000]), Ok(6000));
    }

    #[test]
    fn empty_order_has_shipping() {
        assert_eq!(total_with_shipping(&[]), Ok(500));
    }

    #[test]
    fn overflow_is_an_error() {
        assert!(total_with_shipping(&[u64::MAX, 1]).is_err());
    }
}
