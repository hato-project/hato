# hato

[![Build Status](https://travis-ci.org/hato-project/hato.svg?branch=master)](https://travis-ci.org/hato-project/hato)
[![Run Status](https://api.shippable.com/projects/5bc44b5564e8ed070004cb2e/badge?branch=master)]()


## Development
Auto-compiling and reloading server after saving.

1.install tools
```
cargo install systemfd cargo-watch
```

2.start development server   
```
systemfd --no-pid -s http::8000 -- cargo watch -x 'run -- server'
```
