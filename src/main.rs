extern crate clap;
extern crate crossbeam_channel as channel;
extern crate graphql_parser;
extern crate num_cpus;
extern crate serde_json;

mod cli;
mod graphql;
mod typescript;
mod worker_pool;

fn main() {
    let config = cli::Config::from_cli();
    let schema = graphql::parse_schema(&config.schema_path).expect("Failed to parse schema");
    let worker_pool = worker_pool::WorkerPool::new(config.number_threads, schema);
    let result = worker_pool.work(&config.root_dir, config.use_custom_scalars);
    cli::print_work_result(result);
}
