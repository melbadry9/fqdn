# fqdn
Extract root and sub domain from FQDN

## Install


- Intall from `github`
```bash
git clone https://github.com/melbadry9/fqdn.git
cd fqdn
cargo install --path .
```

## Usage

- Help

```txt
Extract Root-domain/Sub-domain 0.1
Mohamed Elbadry <me@melbadry9.xyz>

USAGE:
    fqdn [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -j, --json       Sets output format to json      
    -V, --version    Prints version information      

OPTIONS:
    -d, --domain <DOMAIN>      Sets domain to check  
    -t, --threads <THREADS>    Sets number of threads
  ```



```bash
melbadry9@localhost:/test$ fqdn -d ssl.a.example.com -j | jq

{
  "root_domain": "example.com",     
  "sub_domain": "ssl.a"
}
```


```bash
melbadry9@localhost:/test$ cat domains.txt | fqdn

example1.com
example2.com
``` 
