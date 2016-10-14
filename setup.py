try:
    from setuptools import setup
except ImportError:
    from distutils.core import setup
from rust_ext import build_rust_cmdclass, install_lib_including_rust


setup(
    name='ryaml',
    version='0.1.0',
    cmdclass={
        # This enables 'setup.py build_rust', and makes it run
        # 'cargo extensions/cargo.toml' before building your package.
        'build_rust': build_rust_cmdclass('extensions/Cargo.toml'),
        # This causes your rust binary to be automatically installed
        # with the package when install_lib runs (including when you
        # run 'setup.py install'.
        'install_lib': install_lib_including_rust
    },
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    install_requires=[
        "rust-ext>=0.1"
    ],
    dependency_links=[
        "git+https://github.com/novocaine/rust-python-ext.git@81674f34ddb8f6fe23d610df93a4c4b57daa222b#egg=rust-ext-0.1"
    ]
)
