from setuptools import setup

setup(
    name="skv",
    version="0.1",
    description="Sketch viewer",
    url="http://github.com/d-dorazio/lart",
    author="Daniele D'Orazio",
    author_email="d.dorazio96@gmail.com",
    license="MIT",
    packages=["skv"],
    zip_safe=False,
    entry_points={
        "console_scripts": ["skv=skv.skv:main"],
    },
    install_requires=["PySide6>=6.3"]
)
