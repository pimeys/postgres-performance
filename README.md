# PostgreSQL performance experiments

Prerequisites:

- Docker and docker-compose
- Rust and cargo, tested with 1.55.0 and nightly, but older versions might work.

## Running the experiment

Start the databases and monitoring:

``` bash
> docker-compose up
```

Run the benchmarks:

``` bash
> cd connect-test
> cargo bench
```
