rust-web-demo
====

### env
```sh
$ cargo --version
 cargo 0.9.0-nightly (8fc3fd8 2016-02-29)

$ rustc --version
 rustc 1.8.0 (db2939409 2016-04-11)

$ uname -a
 Linux ubuntu 4.2.0-16-generic #19-Ubuntu SMP Thu Oct 8 15:35:06 UTC 2015 x86_64 x86_64 x86_64 GNU/Linux

$ sudo apt-get install postgresql redis-server
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
