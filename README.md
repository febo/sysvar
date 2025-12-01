<h1 align="center">
  <code>sysvar</code>
</h1>
<p align="center">
  <img width="400" alt="playground" src="https://github.com/user-attachments/assets/e4cf1712-e423-4dbd-8b15-5d8f3d18ffed" />
</p>

<p align="center">
  Benchmark for retrieving sysvar data.
</p>

## Overview

There are two ways to retrieve sysvar data. Many of the sysvar have an individual syscall (e.g., `Rent`
has a `sol_get_rent_sysvar`) and there is also a generic `sol_get_sysvar`. Apart from the base syscall base cost,
the genetic syscall charges `max(size_of::<T>() / 250, 10)`, while syvar-specific ones charges `size_of::<T>()`.
For any sysvar longer than `10` bytes, the generic syscall is cheaper in terms of compute units.

## Building and Running

A [`Makefile`](./Makefile) is provided with basic commands to:
* `all`: build all programs &mdash; his is required before running the benchmark.
* `bench`: run the benchmark.
* `build-sbf-%`: build an individual program &mdash; `%` is the serialization program name (e.g., `programs-transmute`).
* `tests`: run the tests.

To execute the benchmark, it is first necessary to build all programs:
```bash
make all
```

Followed by:
```bash
make bench
```

After the execution, mollusk will report the compute units in a `compute_units.md`
located at `./target/benches`.
```
| Name                | CUs | Delta |
|---------------------|-----|-------|
| current::sysvar_get | 135 |   --  |
| generic::sysvar_get | 124 |   --  |
```

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
