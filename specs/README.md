# YAML Configuration Specification
version: "0.1"

builder: 
  - "your_builder_image_tag"
  - "your_builder_image_tag2"

env:
  - FOO: bar
    BAR: bazz
  - QUX: quux

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

before_install:
  - command 1
  - command 2


install:
  - command 1
  - command 2

build:
  before_script:
    - command 1
    - command 2
  script:
    - command 1
    - command 2
  after_script:
    - command 1
    - command 2
  on_success:
    - command 1
    - command 2
  on_failure:
    - command 1
    - command 2
test:
  before_script:
    - command 1
    - command 2
  script:
    - command 1
    - command 2
  after_script:
    - command 1
    - command 2
  on_success:
    - command 1
    - command 2
  on_failure:
    - command 1
    - command 2

matrix:
  include:
  - builder: "builder_env_image_tag_1"
    env: FOO=bar
  - name: "test 2"
    builder: "builder_env_image_tag_2"
    env: FOO=baz
  exclude:
  - env: FOO=bar
  - name: "test 2"
    builder: "builder_env_image_tag_2"
    env: FOO=baz
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