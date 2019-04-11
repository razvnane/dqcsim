from setuptools import setup, Extension
import os

try:
    os.environ['DQCSIM_RUST_RELEASE']
    libdir = '../../target/release'
except KeyError:
    libdir = '../../target/debug'

setup(
    name = 'dqcsim',
    version = '1.0', # TODO: retrieve from cargo
    description = 'Python bindings for DQCsim, the Delft Quantum & Classical Simulator',
    author = 'TU Delft Quantum & Computer Architecture, QuTech',
    packages = [
        'dqcsim',
        'dqcsim.common',
    ],
    package_dir = {'dqcsim': 'dqcsim'},
    ext_modules = [Extension(
        'dqcsim._dqcsim',
        ['gen/dqcsim.c'],
        library_dirs = [libdir],
        runtime_library_dirs = [os.environ['DQCSIM_HOME'] + '/lib'],
        libraries = ['dqcsim'],
        extra_compile_args = ['-std=c99']
    )],
    install_requires=[
        'cbor',
    ],
    test_suite='nose.collector',
    tests_require=['nose'],
)
