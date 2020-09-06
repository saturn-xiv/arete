# ARETE

A web application by C++ and React.

## Usage

- Start alpine

```bash
$ docker pull alpine:latest
$ docker build -t arete x64
$ docker run --rm -it --network host -v `pwd`:/workspace arete
```

- Build

```bash
$ mkdir build
$ cd build
$ CXX=clang++ CC=clang cmake -DCMAKE_BUILD_TYPE=Release ..
$ make -j
```

## Editor

### VSCode

- C/C++
- CMake Tools
- Docker

## Documents

- [CMake](https://cmake.org/cmake/help/latest/guide/tutorial/index.html)
- [Google C++ Style Guide](https://google.github.io/styleguide/cppguide.html)
- [Boost](https://www.boost.org/doc/libs/release/more/getting_started/)
- [mysql++](https://tangentsoft.com/mysqlpp/doc/html/userman/index.html)
- [libpqxx](http://pqxx.org/development/libpqxx/)
- [sqlite3](https://www.sqlite.org/cintro.html)
- [hiredis](https://github.com/redis/hiredis)
- [libsodium](https://doc.libsodium.org/)
