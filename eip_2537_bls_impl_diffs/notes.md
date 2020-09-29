|              | G1+ | G1* | G1^^ | G2+ | G2* | G2^^ | P2  | P4  | P8   | MAPG1 | MAPG2 |
|--------------|-----|-----|------|-----|-----|------|-----|-----|------|-------|-------|
| EIP GAS      | 600 |  12k|      |4500 |55k  |      |138k | 181k|184k  |5500   |110k   |
| MLG EIP csv  | ✓   | ✓   | *    |✓    | ✓   | *    | ✓   | ✓   | ✓    | X     | X     |
| MLG          | 9us |406us| -    |20us |986us| -    |3.1ms|5.0ms|7.0ms | -     | -     |
| Matter-labs  |12us |255us| -    |24us |1.2ms| -    |6.1ms|9.3ms|17ms  | -     | -     |
| Paired       | 6us |337us| *    |15us |1.1us| *    |5.1ms|9.9ms|17ms  |117us  |2.23ms |
| Kilic        | 5us |176us| -    |12us |572us| -    |2.9ms|5.1ms|9.4ms | -     | -     |

(caution, very preliminar performance tests, only to check big diffs)

-           = Not tested
*           = Not implemented
X           = Vector tests fails (does not celean subgroup)
MLG EIP csv = Milagro passing eip2537 test vectors
MLG         = Milagro timings
Matter-labs = Matter labs eip1962 timings
Kilic       = Afais, golang choosen impl