# rust_hashmap_gem

This library provides the feature to use [Rust's HashMap](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html) from Ruby.

## benchmark

Sadly, this is slower than the original one.

```
$ ruby benchmark.rb --rust --seed 1
Hash: Rust
Seed: 1
       user     system      total        real
values  1.967790   0.231081   2.198871 (  2.201430)
keys    1.474711   0.082201   1.556912 (  1.557567)
find    5.707038   0.001190   5.708228 (  5.709953)
insert 10.242965   0.013552  10.256517 ( 10.260209)
delete  5.724710   0.001494   5.726204 (  5.728144)
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
