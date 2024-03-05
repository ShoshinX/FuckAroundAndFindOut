use rand::prelude::*;
use tracing::{event, instrument, Level};
use tracing_subscriber::FmtSubscriber;

pub enum RunnerState {
    FAIL,
    PASS,
    UNRESOLVED,
}

// fuzz generates the input string
// run generates the input string and gives it to a function to run
// runs generates the input string and gives it to a function to run a set number of times

pub trait Fuzzer {
    fn fuzz(&self) -> String;
    fn run(&self, input_program: String) -> String;
    fn runs(&self, input_program: String) -> String;
}

// okay this is cool
#[instrument]
fn fuzzer(max_length: u32, char_start: u8, char_range: u8) -> String {
    event!(Level::INFO, "inside my_function!");
    let mut rng: ThreadRng = thread_rng();
    let x: u32 = rng.gen_range(0..max_length);
    let mut out: String = "".to_owned();
    for _ in 0..x {
        out.push(rng.gen_range(char_start..char_start + char_range) as char)
    }
    out
}

pub trait Runner {
    fn run(&self, input: String) -> (String, RunnerState);
}

fn main() {
    let subscriber = FmtSubscriber::new();
    let _ = tracing::subscriber::set_global_default(subscriber);
    println!("Hello, world! {:?}", fuzzer(10, b'a', 32));
}
