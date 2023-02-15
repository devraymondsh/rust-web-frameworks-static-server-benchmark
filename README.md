# Rust web frameworks benchmark for static file serving

#### Benchmarks on Intel core i5 12400:
|  Name  | Version    | Requests/sec | Transfer/sec |
|:------:|------------|:------------:|:------------:|
| Gotham | 0.7.1      |    1840.79   |    9.02GB   |
| Hyper  | 0.14.23    |    1831.10   |    8.97GB   |
| Axum   | 0.6.4      |    1816.04   |    8.90GB   |
| Warp   | 0.3.3      |    1814.85   |    8.90GB   |
| Actix  | 4.3.0      |    1497.27   |    7.32GB   |
| Rocket | 0.5.0-rc.2 |    1127.21   |    5.54GB   |


## **Requirements:**
### Docker: [Get Docker](https://docs.docker.com/get-docker/)
### Rust & Cargo: [Rustup](https://rustup.rs/)

## **Running the benchmark:**
### **Build**
#### Enter the following command to start:
```bash
docker build -t devraymondsh/rust_web_frameworks_benchmark .
```
### **Run with default settings**
#### Enter the following command to start:
```bash
docker run -v="$(pwd)/logs:/rust_web_frameworks_benchmark/logs" --rm -it devraymondsh/rust_web_frameworks_benchmark
```

### **Run with custom settings**
#### Enter the following command to start:
```bash
docker run -e="FIBO_TARGET=20" -e="RATE=50000" -e="THREADS=8" -e="CLIENTS=200" -e="DURATION=10" -v="$(pwd)/logs:/rust_web_frameworks_benchmark/logs" --rm -it devraymondsh/rust_web_frameworks_benchmark
```
#### Feel free to replace values with your own desire. Keep in mind the DURATION env is in seconds.

<br />

#### NOTE: You can find the logs in `logs` directory after running the benchmark.
