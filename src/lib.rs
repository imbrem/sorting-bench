use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct PriceLevel {
    pub price: u32,
    pub quantity: u32,
    pub exchange_id: u32,
    pub order_id: u32,
}

extern "C" {
    pub fn sort_price_levels_c(data: *mut PriceLevel, len: usize);
    pub fn sort_price_levels_cpp(data: *mut PriceLevel, len: usize);
    pub fn sort_price_levels_c_cpp(data: *mut PriceLevel, len: usize);
}

#[inline(always)]
fn compare_price_levels(left: &PriceLevel, right: &PriceLevel) -> Ordering {
    left.price
        .cmp(&right.price)
        .then(left.quantity.cmp(&right.quantity))
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn sort_price_levels_rust(data: *mut PriceLevel, len: usize) {
    let slice = std::slice::from_raw_parts_mut(data, len);
    slice.sort_unstable_by(compare_price_levels)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn c_sort() {
        unsafe { sorting_test(sort_price_levels_c) }
    }

    #[test]
    fn cpp_sort() {
        unsafe { sorting_test(sort_price_levels_cpp) }
    }

    #[test]
    fn c_cpp_sort() {
        unsafe { sorting_test(sort_price_levels_c_cpp) }
    }

    #[test]
    fn rust_sort() {
        unsafe { sorting_test(sort_price_levels_rust) }
    }

    unsafe fn sorting_test(sorter: unsafe extern "C" fn(*mut PriceLevel, usize)) {
        let mut levels = [
            PriceLevel {
                price: 5,
                quantity: 100,
                exchange_id: 2,
                order_id: 42,
            },
            PriceLevel {
                price: 3,
                quantity: 100,
                exchange_id: 88,
                order_id: 32,
            },
            PriceLevel {
                price: 1,
                quantity: 50,
                exchange_id: 39,
                order_id: 40,
            },
            PriceLevel {
                price: 1,
                quantity: 100,
                exchange_id: 32,
                order_id: 12,
            },
        ];
        let len = levels.len();
        unsafe { sorter(levels.as_mut_ptr(), len) };
        assert_eq!(
            levels,
            [
                PriceLevel {
                    price: 1,
                    quantity: 50,
                    exchange_id: 39,
                    order_id: 40,
                },
                PriceLevel {
                    price: 1,
                    quantity: 100,
                    exchange_id: 32,
                    order_id: 12,
                },
                PriceLevel {
                    price: 3,
                    quantity: 100,
                    exchange_id: 88,
                    order_id: 32,
                },
                PriceLevel {
                    price: 5,
                    quantity: 100,
                    exchange_id: 2,
                    order_id: 42,
                },
            ]
        );
    }
}
