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

---

```
$ rake clobber install
$ ruby benchmark.rb --rust --seed 1
Hash: Rust
Seed: 1
       user     system      total        real
values  1.896433   0.152249   2.048682 (  2.049503)
keys    1.589012   0.118299   1.707311 (  1.708012)
find   11.612382   0.002426  11.614808 ( 11.617854)
insert 15.397090   0.012138  15.409228 ( 15.413532)
delete  7.581418   0.001750   7.583168 (  7.585473)
```

```
$ METHOD_CACHE=1 rake clobber install
$ ruby benchmark.rb --rust --seed 1
Hash: Rust
Seed: 1
       user     system      total        real
values  1.924479   0.166750   2.091229 (  2.092445)
keys    1.514441   0.112787   1.627228 (  1.628187)
find    8.677083   0.001990   8.679073 (  8.681669)
insert 12.661791   0.010463  12.672254 ( 12.678383)
delete  5.908580   0.001670   5.910250 (  5.912530)
```
