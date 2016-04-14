rust-web-demo
====

### env
```sh
$ cargo --version
 cargo 0.8.0-nightly (28a0cbb 2016-01-17)

$ rustc --version
 rustc 1.7.0 (a5d1e7a59 2016-02-29)

$ uname -a
 Linux u 4.2.0-30-generic #36-Ubuntu SMP Fri Feb 26 00:58:07 UTC 2016 x86_64 x86_64 x86_64 GNU/Linux

$ git clone https://github.com/hikelee/rust-web-demo.git

$ cd rust-web-demo

#run init.sql in postgresql client to init table/data
$ cat web-root/sql/init.sql

$RUST_LOG=rust_web_demo=info cargo run
```

### test
```sh
#go to below url, login with admin/admin
$ chrome http://localhost:8080/

```
