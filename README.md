# 2d-vector-timings

`cargo run --release`

Typical results on i5-6200U:

```
vector of vector fetch (10000 x 10000): 394 +/- 75 ns
vector of array fetch (10000 x 10000): 326 +/- 75 ns
vector of box array fetch (10000 x 10000): 247 +/- 74 ns
```

```
vector of vector fetch (10000 x 10000): 497 +/- 80 ns
vector of array fetch (10000 x 10000): 312 +/- 77 ns
vector of box array fetch (10000 x 10000): 527 +/- 77 ns
```
