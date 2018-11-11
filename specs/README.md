# YAML Configuration Specification
version: "0.1"

builder: your_builder_image

env:
  - FOO=bar
  - BAZ=qux

branches:
  only:
    - master
    - /^.*?-release$/
  
  except:
    - master
    - /^.*?-release$/

tags:
  only:
    - v1.1
    - /^v0\..*?$/
  
  except:
    - v2.0
    - /^except-tag$/

build:
  before:
    - command 1
    - command 2
  ci:
    - command 1
    - command 2
  after:
    - command 1
    - command 2
  on_success:
    - command 1
    - command 2
  on_failure:
    - command 1
    - command 2
  push:
    docker_tag: my/dockerhub_image

notifications:
  webhooks:
    urls:
      - http://example.com/ci_webhook
    on_success: change
    on_failure: always
  email:
    recipients:
      - you@example.com
    on_success: never
    on_failure: always