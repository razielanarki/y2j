Yaml to Json converter (y2j)
============================

Simple binary taking some yaml from standard input and writing Json equivalent
to standard output.

Why this project
----------------

I needed a small binary to convert Yaml to Json and pipe it to `jq`. All the
solutions out there requires to install Python or Ruby or Node, I don't need
a 30MB VM to do such a simple thing! That's why I wrote those 10 lines of Rust.
