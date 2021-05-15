import sys

from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="py-randomprime",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Rust",
    ],
    packages=["py_randomprime"],
    install_requires=[
    ],
    rust_extensions=[
        RustExtension(
            "py_randomprime.rust",
            binding=Binding.PyO3,
            py_limited_api=True,
        )
    ],
    include_package_data=True,
    zip_safe=False,
)