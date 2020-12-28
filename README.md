# rust_hashmap_gem

This library provides the feature to use [Rust's HashMap](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html) from Ruby.

## benchmark

Sadly, this is slower than the original one.

```
$ ruby benchmark.rb --rust --seed 1
Hash: Rust
Seed: 1
       user     system      total        real
values  1.982131   0.261201   2.243332 (  2.244513)
keys    1.452019   0.088894   1.540913 (  1.547603)
find    4.916835   0.001163   4.917998 (  4.919878)
insert  9.426069   0.010357   9.436426 (  9.439591)
delete  4.813639   0.001478   4.815117 (  4.816879)
```

```
$ ruby benchmark.rb --seed 1
Hash: Original
Seed: 1
       user     system      total        real
values  1.219877   0.184663   1.404540 (  1.405405)
keys    0.601086   0.096397   0.697483 (  0.697947)
find    2.884605   0.000930   2.885535 (  2.887535)
insert 14.462791   0.015545  14.478336 ( 14.484709)
delete  4.118091   0.001139   4.119230 (  4.120935)
```
