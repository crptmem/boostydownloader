boostydownloader
================
A simple application to bulk-download content from Boosty.
Made with my own library boosty-rs (https://github.com/crptmem/boosty-rs)

Installation
=====
  $ cargo install boostydownload

Usage
=====
  $ boostydownload --blog BLOG_NAME
By default content is downloaded to `$PWD/img`. You can change path by `--path` argument.

Obtaining access token
======================
Go to https://boosty.to, open developer tools in your browser,
go to Storage (Application) -> Cookies. You need `auth`, click on it and in
right panel click RMB on `accessToken` and copy it.

Usage
=====
 Without authorization:
  $ boostydownload --blog USERNAME
 With authorization:
  $ boostydownload --blog USERNAME --access-token ACCESS_TOKEN
 If requested blog have more than 300 posts:
  $ boostydownload --blog USERNAME --limit POSTS_COUNT
