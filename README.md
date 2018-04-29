# RYAML - Python YAML library in pure Rust

This is an implementation of a YAML parser and writer in pure Rust to be used with Python which is
considerabily faster that the standard [PyYAML](http://pyyaml.org/) library with the `libyaml`
support.

> Note that this code is not production ready and most of the features of `PyYAML` are not
> implemented yet

## Build

To build this package Rust nighly should be installed in your system. The best option is to install
it using [rustup](https://www.rustup.rs/) by just following the instruction to install the tool on
your system.

The crate `rust-cpython` uses the
[interpolate-ident](https://github.com/SkylerLipthay/interpolate_idents) crate which uses some
features available only on the _nightly_ branch of the `rustc` compiler so you will need to install
a specific version to be able to compile the code:

```bash
rm -rf extensions/target/  # Ensure that no precompiled code is left
rustup default nightly
```

Then you need to install the build requirements becasue we'll use the
[rust-python-ext)](https://github.com/novocaine/rust-python-ext) to compile the Rust code during the
installation:

```bash
pip install -r requirements_build.txt
```

Last step build and install the `ryaml` package:

```bash
python setup.py install
```

We can test it quickly form the Python's REPL:

```
Python 2.7.12 (default, Oct 11 2016, 14:35:52)
[GCC 4.2.1 Compatible Apple LLVM 8.0.0 (clang-800.0.38)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import ryaml
>>> ryaml
<module 'ryaml' from '/Users/danieleesposti/.virtualenvs/ryaml/lib/python2.7/site-packages/ryaml-0.1.0-py2.7.egg/ryaml.so'>
>>>
```

As you can see the `ryaml` module is our compiled dinamic library.

## Testing

To run the unit tests, assuming that you already installed the `ryaml` package by following the
instructions in the **Build** section, just run:

```bash
pip install -r requirements_dev.txt
py.test -xv tests.py
```

## Benchmarks

A small benchmark can be found in the `benchmakrs` directory and can be launched by:

```bash
python benchmarks/run.py
```

The benchmark is very simple, it just loads a bunch of data generated by
[Mokckaroo](https://www.mockaroo.com/) and check the amount of time spent in parsing and dumping
the YAML using the [PyYAML](http://pyyaml.org/) package compiled with the C extension and `ryaml`.

Here the result on my MacBookPro 13" Early 2015:

```bash
$ python benchmark/run.py
Loading test data...
Starting benchmark...
yaml decode time: 0:00:06.572454
yaml encode time: 0:00:02.235445
ryaml decode time: 0:00:00.740896
ryaml encode time: 0:00:00.735613
```
