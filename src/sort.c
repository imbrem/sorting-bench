#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

// Struct representing a (price, quantity) pair
typedef struct {
    uint32_t price;
    uint32_t quantity;
    uint32_t exchange_id;
    uint32_t order_id;
} PriceLevel;

// Comparison function for qsort
inline static int compare_levels(const void *a, const void *b) {
    const PriceLevel *levelA = (const PriceLevel*)a;
    const PriceLevel *levelB = (const PriceLevel*)b;

    // Compare based on price, followed by quantity
    if (levelA->price < levelB->price)
        return -1;
    else if (levelA->price > levelB->price)
        return 1;
    else if (levelA->quantity < levelB->quantity)
        return -1;
    else if (levelA->quantity > levelB->quantity)
        return 1;
    else
        return 0;
}

void sort_price_levels_c(PriceLevel* data, size_t len) {
    qsort(data, len, sizeof(PriceLevel), compare_levels);
}