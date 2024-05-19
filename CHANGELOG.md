# Changelog

## [0.6.1](https://github.com/helton/hbox/compare/v0.6.0...v0.6.1) (2024-05-19)


### Bug Fixes

* update ports in list command ([246ed50](https://github.com/helton/hbox/commit/246ed509600cdf1407b460c09a86af0394362776))

## [0.6.0](https://github.com/helton/hbox/compare/v0.5.0...v0.6.0) (2024-05-19)


### Features

* add command hbox config ([86c10e0](https://github.com/helton/hbox/commit/86c10e052e0953e89d9b4be55fbb93daf397775e))
* add support to Podman as container engine ([2f0f5b2](https://github.com/helton/hbox/commit/2f0f5b2a3bb9c818134f3f4215ccda2ab9debcc3))
* add support to ports mapping ([cb8eef7](https://github.com/helton/hbox/commit/cb8eef7969225ae042febe45becf08aaabb7ace1))

## [0.5.0](https://github.com/helton/hbox/compare/v0.4.0...v0.5.0) (2024-05-19)


### Features

* add index directory separated in shards ([5ab15a1](https://github.com/helton/hbox/commit/5ab15a1d6c1c52d197349420b5e7ccec391bb7cd))
* add logging ([92cadd4](https://github.com/helton/hbox/commit/92cadd479bf3e421e118da58dfb5fb4aa2d39748))
* add separate log files for debug and trace ([c9c9617](https://github.com/helton/hbox/commit/c9c96176a6c897addbc89c0d87394dd729027a2f))
* add separate log files for debug and trace ([17d1952](https://github.com/helton/hbox/commit/17d19523e844efa00c02eb64347d0c7be2d4480b))
* add separate log files for debug and trace ([9322a13](https://github.com/helton/hbox/commit/9322a13436a30e40015c1f714c43d5f8b62a1533))
* add stdout and stderr capture as experimental features ([96f8391](https://github.com/helton/hbox/commit/96f83917942bfdab83cedc8741a8fdd8aeb6b1b5))
* expand env vars ([3b95eaf](https://github.com/helton/hbox/commit/3b95eafc707e8428158122094d2ce71c9c116c90))
* generate container name with info to better track execution ([264ff83](https://github.com/helton/hbox/commit/264ff8350ebaaac805a849fbe153cc17f2f9d88a))
* keep index and overrides in separate folders ([c54a159](https://github.com/helton/hbox/commit/c54a1597df4a286493d7ec877fdff111fd494ba5))
* redirect command stdout and stderr to logs ([81fa5c0](https://github.com/helton/hbox/commit/81fa5c025ff19febd058b25aab27f256883e254c))
* separate versions.json into individual files ([3f55fb7](https://github.com/helton/hbox/commit/3f55fb724c9f53dcb666ce9d1ee25500561d70b2))
* support extra binary commands and wrap args in quotes ([9e47097](https://github.com/helton/hbox/commit/9e47097a24b9155be5dc8b7ebbf7a3b4fb4b3e39))


### Bug Fixes

* docker pull ([d0831c8](https://github.com/helton/hbox/commit/d0831c863059bcd9ef79d6c2b503158814b11920))
* escape subcommands passed to run ([81f49c3](https://github.com/helton/hbox/commit/81f49c34f8685dde86105930afae6ddc5d79c5e2))
* flags for interactive commands ([079183b](https://github.com/helton/hbox/commit/079183b8a07a9d4acd1ec7f3bf1f0b7effb64086))
* handle case where version folder doesn't exists ([ea2500b](https://github.com/helton/hbox/commit/ea2500bcb98b162e115199ad697c61c52b5ea3ab))

## [0.4.0](https://github.com/helton/hbox/compare/v0.3.0...v0.4.0) (2024-05-17)


### Features

* add environment variables support ([86bf605](https://github.com/helton/hbox/commit/86bf6052fb4804cad75c5c6c5b57bc07b6651ffc))
* add verbose mode to list command ([cc330a0](https://github.com/helton/hbox/commit/cc330a0f532bda7ad5dbd59a67667425f0e0183a))
* add way to access and shim internal binaries ([633a9c2](https://github.com/helton/hbox/commit/633a9c2b1abf3b3a891b50ac4f302553c9820193))

## [0.3.0](https://github.com/helton/hbox/compare/v0.2.0...v0.3.0) (2024-05-17)


### Features

* add --rm flag for every docker run ([4964841](https://github.com/helton/hbox/commit/4964841f54a8e57c92fea921cad39d35197116cc))

## [0.2.0](https://github.com/helton/hbox/compare/v0.1.1...v0.2.0) (2024-05-16)


### Features

* add support to set current directory ([866b807](https://github.com/helton/hbox/commit/866b807f6f47a59b6472f724b63679371c38be8b))

## [0.1.1](https://github.com/helton/hbox/compare/v0.1.0...v0.1.1) (2024-05-14)


### Bug Fixes

* update action versions and codecov ([e3fad6a](https://github.com/helton/hbox/commit/e3fad6a3d0b0c11fb00ae607f8d5fcec7c0c5766))

## 0.1.0 (2024-05-14)


### Features

* add run command ([9422f50](https://github.com/helton/hbox/commit/9422f507810aafd79237858615affe073e30ade3))
* add support to shell expansion ([cd51ad8](https://github.com/helton/hbox/commit/cd51ad8c11d8c58c8c2f1f0bab1f30934499b201))
* initial version ([c74b7e6](https://github.com/helton/hbox/commit/c74b7e6b2dc7e3984973f05c9b953390e2a90bb5))
