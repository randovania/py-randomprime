import sys

from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="randomprime",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Rust",
    ],
    packages=["randomprime"],
    install_requires=[
    ],
    rust_extensions=[
        RustExtension(
            "randomprime.randomprime",
            path="randomprime/Cargo.toml",
            binding=Binding.NoBinding,
        )
    ],
    include_package_data=True,
    zip_safe=False,
)