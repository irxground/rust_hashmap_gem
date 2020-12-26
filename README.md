# rust_hashmap_gem

This library provides the feature to use [Rust's HashMap](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html) from Ruby.

## benchmark

Sadly, this is slower than the original one.

```
$ ruby benchmark.rb --rust --seed 1
Hash: Rust
Seed: 1
       user     system      total        real
values  1.885191   0.164441   2.049632 (  2.052237)
keys    1.461595   0.054283   1.515878 (  1.516320)
find   11.256652   0.002255  11.258907 ( 11.261749)
insert 15.286804   0.013948  15.300752 ( 15.305879)
delete  7.310167   0.001280   7.311447 (  7.312999)
```

```
$ ruby benchmark.rb --seed 1
Hash: Original
Seed: 1
       user     system      total        real
values  1.100674   0.150917   1.251591 (  1.258886)
keys    0.526004   0.042145   0.568149 (  0.570534)
find    2.607541   0.002803   2.610344 (  2.616013)
insert 17.317054   0.012727  17.329781 ( 17.335842)
delete  3.717545   0.000869   3.718414 (  3.719866)
```
