# YAML Configuration Specification
version: "0.1"

env:
  - FOO=bar
  - BAZ=qux

branches:
  only:
    - master
    - /^.*?-release$/

build:
#todo

notifications:
  webhooks:
    urls:
      - http://example.com/ci_webhook
