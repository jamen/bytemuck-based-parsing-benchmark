# bytemuck-based-parsing-benchmark

```
ref_option_parse        time:   [6.5560 µs 6.5767 µs 6.6025 µs]
                        change: [+0.1481% +0.8333% +1.5457%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe

ref_result_parse        time:   [6.5260 µs 6.5328 µs 6.5397 µs]
                        change: [-0.8352% -0.2125% +0.2104%] (p = 0.53 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

nomlike_min_result_parse
                        time:   [6.5498 µs 6.5680 µs 6.5885 µs]
                        change: [-0.3609% +0.2084% +0.6094%] (p = 0.49 > 0.05)
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe

nomlike_full_result_parse
                        time:   [6.4996 µs 6.5079 µs 6.5174 µs]
                        change: [-0.7765% -0.5877% -0.3909%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

nomlike_option_parse    time:   [6.5991 µs 6.6290 µs 6.6697 µs]
                        change: [+0.8114% +1.0925% +1.3950%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
```

AMD Ryzen 5 2600