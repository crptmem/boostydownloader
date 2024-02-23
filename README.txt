boostydownloader
================
A simple application to bulk-download content from Boosty

Build
=====
  $ git clone https://github.com/crptmem/boostydownloader.git && cd boostydownloader
  $ cargo build

Obtaining access token
======================
Go to https://boosty.to, open developer tools in your browser,
go to Storage (Application) -> Cookies. You need `auth`, click on it and in
right panel click RMB on `accessToken` and copy it.

Usage
=====
 Without authorization:
  $ boostydownloader --blog USERNAME
 With authorization:
  $ boostydownloader --blog USERNAME --access-token ACCESS_TOKEN
