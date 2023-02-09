# Rust web frameworks benchmark

#### Benchmarks on Intel core i5 12400:
|  Name  | Version    | Requests/sec | Transfer/sec | 50% Latency Distribution | 75% Latency Distribution | 99% Latency Distribution |
|:------:|------------|:------------:|:------------:|:------------------------:|:------------------------:|:------------------------:|
| Warp   | 0.3.3      |   22322.22   |    12.92MB   |          8.30ms          |          11.97ms         |          25.53ms         |
| Salvo  | 0.37.9     |   22172.96   |    13.15MB   |          8.32ms          |          12.10ms         |          27.75ms         |
| Gotham | 0.7.1      |   22094.69   |    13.89MB   |          8.35ms          |          12.07ms         |          27.29ms         |
| Hyper  | 0.14.23    |   21994.87   |    12.73MB   |          8.38ms          |          12.09ms         |          25.68ms         |
| Axum   | 0.6.4      |   21796.71   |    12.62MB   |          8.46ms          |          12.29ms         |          26.61ms         |
| Rocket | 0.5.0-rc.2 |   21482.86   |    14.85MB   |          8.51ms          |          12.52ms         |          29.67ms         |
| Actix  | 4.3.0      |   14665.74   |    8.49MB    |          13.56ms         |          15.53ms         |          27.25ms         |


## **Requirements:**
### Docker: [Get Docker](https://docs.docker.com/get-docker/)
### Rust & Cargo: [Rustup](https://rustup.rs/)

## **Running the benchmark:**
### **Build**
#### Enter the following command to start:
```bash
docker build -t devraymondsh/rust_web_frameworks_benchmark .
```
### **Run**
#### Enter the following command to start:
```bash
docker run -v="$(pwd)/logs:/rust_web_frameworks_benchmark/logs" --rm -it devraymondsh/rust_web_frameworks_benchmark
```

#### You can find the logs in `logs` directory after running the benchmark.
