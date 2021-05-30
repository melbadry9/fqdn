use clap::{App, Arg};
use std::io::{self, BufRead};
use threadpool::ThreadPool;
use addr::parser::DnsName;
use addr::psl::List;

fn main() {
    let args = App::new("Check Domain Availability")
        .version("0.1")
        .author("Mohamed Elbadry <me@melbadry9.xyz>")
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .value_name("THREADS")
                .help("Sets number of threads")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("domain")
                .short("d")
                .long("domain")
                .value_name("DOMAIN")
                .help("Sets domain to check")
                .takes_value(true),
        )
        .get_matches();

    if !(args.is_present("domain")) {
        let stream = io::stdin();
        let pool = ThreadPool::new(
            args.value_of("threads")
                .unwrap_or("3")
                .parse::<usize>()
                .unwrap_or(3),
        );
        for domain in stream.lock().lines() {
            pool.execute(|| check_av(&domain.unwrap()));
        }
        pool.join();
    } else {
        check_av(&args.value_of("domain").unwrap().to_string())
    }
}

fn check_av(domain: &String) {
    let dom = List.parse_dns_name(domain).unwrap();
    println!("{}", dom.root().unwrap());
}