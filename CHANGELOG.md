# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## [Unreleased]

This removes a previously-required dependency on `reqwest`. This should make it
easier to write a client that uses one of the other common HTTP request
libraries.

Breaking changes:

- Removed the `stytch::Client` type. To use its replacement, enable the
  `reqwest` feature and use `stytch::reqwest::Client` and see the next item.
- Removed the `send` method from all requests. Use `req.build()` to build a
  `stytch::Request` and pass it to a client.

  ```diff
  - req.send(client).await
  + client.send(req.build()).await
  ```

## v0.1.1 - 2022-09-24

Fixes:

- Implement `Default` for `Attributes` to make it easier to build requests and responses that don't use them.
- Make all `Attributes` fields public.
- Make `AuthenticationFactor.factor` public.


## v0.1.0 - 2022-07-18

Initial commit! This was extracted from an authentication module I'm using for
another project.

This has early, incomplete support for the following endpoints:

- /v1/magic_links/authenticate
- /v1/magic_links/email/send
- /v1/sessions/authenticate
