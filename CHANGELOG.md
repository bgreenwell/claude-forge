# changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### added
- Initial implementation of `cforge` CLI tool.
- `init` command to scaffold a new marketplace.
- `new-plugin` command to scaffold a new plugin.
- `add` command with subcommands for `skill`, `command`, `agent`, and `hook`.
- `validate` command to check plugin integrity.
- `register` command to add plugins to the marketplace manifest.
- `--description` / `-d` flag for `init`, `new-plugin`, and `add` commands to bypass interactive prompts.