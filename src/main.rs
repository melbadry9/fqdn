use clap::{App, Arg};
use std::io::{self, BufRead};
use threadpool::ThreadPool;
use addr::parser::DnsName;
use addr::psl::List;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
 struct DomName {
     root_domain: String,
     sub_domain: String
 }

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
        .arg(
            Arg::with_name("json")
                .short("j")
                .long("json")
                .value_name("JSON")
                .help("Sets output format to json")
                .takes_value(false),
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
            let opt = args.is_present("json").clone();
            pool.execute(move || check_av(&mut domain.unwrap(), opt ));
        }
        pool.join();
    } else {
        check_av(&mut args.value_of("domain").unwrap().to_string(), args.is_present("json"))
    }
}

fn check_av(domain: &mut String, output: bool) {
    let dom = List.parse_dns_name(domain);
    match dom {
        Ok(dom) => {
            let root = dom.root();
            match root {
                Some(root) => {
                    let mut sub = domain.split(root).collect::<Vec<&str>>()[0].to_string();
                    sub.pop();

                    if output {
                        let dom = DomName {
                            root_domain: root.to_string(),
                            sub_domain: sub
                        };
                        let ser_domain = serde_json::to_string(&dom).unwrap();
                        println!("{}", ser_domain);

                    } else {
                        println!("{}", root)
                    }
                },
                None => {}
            }
        },
        Err(_) => {}
    }
    
}