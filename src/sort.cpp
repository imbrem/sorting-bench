#include <iostream>
#include <algorithm>
#include <cstdint>

// Struct representing a (price, quantity) level
struct PriceLevel {
    uint32_t price;
    uint32_t quantity;
    uint32_t exchange_id;
    uint32_t order_id;
};

// Comparison function for std::sort (branch-free)
inline static bool compare_levels(const PriceLevel& levelA, const PriceLevel& levelB) {
    return (levelA.price < levelB.price) || ((levelA.price == levelB.price) && (levelA.quantity < levelB.quantity));
}

// Comparison function for qsort
inline static int compare_levels_int(const PriceLevel& levelA, const PriceLevel& levelB) {
    // Compare based on price, followed by quantity
    if (levelA.price < levelB.price)
        return -1;
    else if (levelA.price > levelB.price)
        return 1;
    else if (levelA.quantity < levelB.quantity)
        return -1;
    else if (levelA.quantity > levelB.quantity)
        return 1;
    else
        return 0;
}

// Comparison function for std::sort (adapted from C)
inline static bool compare_levels_c(const PriceLevel& levelA, const PriceLevel& levelB) {
    return compare_levels_int(levelA, levelB) < 0;
}

extern "C" {
    void sort_price_levels_cpp(PriceLevel* data, size_t len) {
        std::sort(data, data + len, compare_levels);
    }

    void sort_price_levels_c_cpp(PriceLevel* data, size_t len) {
        std::sort(data, data + len, compare_levels_c);
    }
}