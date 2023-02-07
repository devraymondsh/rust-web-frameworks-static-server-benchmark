# Rust web frameworks benchmark

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