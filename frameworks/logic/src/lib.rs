use async_recursion::async_recursion;
use serde::Serialize;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

#[derive(Clone, Debug, Serialize)]
pub struct FiboResults {
    pub n: u64,
    pub contents: String,
}
impl FiboResults {
    pub fn new(n: u64, contents: String) -> Self {
        FiboResults { n, contents }
    }
}

#[async_recursion]
async fn fibonacci_recursive(n: u64) -> u128 {
    match n {
        0 => panic!("Zero isn't a valid argument to fibonacci_recursive()"),
        1 | 2 => 1,
        3 => 2,
        _ => fibonacci_recursive(n - 1).await + fibonacci_recursive(n - 2).await,
    }
}

pub async fn run_fibo(fibo_destination: String) -> FiboResults {
    match fibo_destination.parse::<u64>() {
        Ok(fibo_destination) => {
            let mut fibo_results: Vec<String> =
                Vec::with_capacity(fibo_destination.try_into().unwrap());

            // Run fibo and store the results in a vector
            for i in 1..=fibo_destination {
                fibo_results.push(format!(
                    "fibonacci ({}) => `{}`",
                    i,
                    fibonacci_recursive(i).await
                ));
            }

            let fibo_results = fibo_results.join("\n");

            // Create a file and store the fibo results in it
            let mut file = File::create("fibo.txt").await.unwrap();
            file.write_all(fibo_results.as_bytes())
                .await
                .unwrap();
            file.shutdown().await.unwrap();

            // Lastly, remove the file
            fs::remove_file("fibo.txt").await.unwrap_or_default();

            FiboResults::new(fibo_destination, fibo_results)
        }
        Err(error) => {
            let error_message = format!(
                "Failed to parse the fibo_destination as u64: {}. The string: '{}'",
                error, fibo_destination
            );

            eprintln!("{}", error_message);

            FiboResults::new(0, String::from(error_message))
        }
    }
}
