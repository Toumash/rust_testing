#![feature(proc_macro_hygiene, decl_macro, once_cell)]
#[macro_use]
extern crate rocket;
use once_cell::sync::Lazy;
use rocket::http::RawStr;
use rocket_prometheus::{
    prometheus::{opts, IntCounterVec},
    PrometheusMetrics,
};

static NAME_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(opts!("name_counter", "Count of names"), &["name"])
        .expect("Could not create lazy IntCounterVec")
});
static EXEC_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(opts!("calls", "Count of names"), &[&"fib"])
        .expect("Could not create lazy IntCounterVec")
});

#[get("/hello/<name>")]
pub fn hello(name: &RawStr) -> String {
    NAME_COUNTER.with_label_values(&[name]).inc();
    format!("Hello, {}!", name)
}

fn main() {
    let prometheus = PrometheusMetrics::new();
    let registry = prometheus.registry();
    registry.register(Box::new(NAME_COUNTER.clone()));
    registry.register(Box::new(EXEC_COUNTER.clone()))
    .unwrap();
    rocket::ignite()
        .attach(prometheus.clone())
        .mount("/", routes![hello])
        .mount("/metrics", prometheus)
        .launch();
    let mut counter = 0;
    fib(10000, &mut counter);
}

fn fib(n: i64, c: &mut i64) -> i64 {
    if n <= 1 {
        return n;
    }
    let name = "fib";
    EXEC_COUNTER.with_label_values(&[&name]).inc();
    return fib(n - 1, c) + fib(n - 2, c);
}
