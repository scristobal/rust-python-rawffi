Rust rawffi bindings for Python

$ cargo build --release
$ cp target/release/librawffi.so rawffi.so
$ python test.py


refs:
- https://docs.python.org/3/c-api/module.html#c.PyModule_Create2