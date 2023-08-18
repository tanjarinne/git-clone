<h1 align="center"><pre>git-clone</pre></h1>

<div align="center">
    <i>A git-clone that keeps your folders happy</i>
</div>
<br>

## About

This tool creates the necessary directories to reflect what you're cloning. For example, cloning this repository with `git-clone git@github.com:tanjarinne/git-clone.git` will create the folders `github.com/tanjarinne` in a configurable path, and then clone the repository.

The tool does not clone by itself, instead it calls `git clone` in the same shell, so `git` must be in your `$PATH` for it to work.

## Disclaimer

This is a tool that I use to keep my folders organised, but it's not thoroughfully tested in different operative systems, shells, etc., and although I tried to write it as clean as possible, there might be conditions that it just won't work. I don't intend to keep developing it much further because it currently does what I need, but I'm open to reviewing PRs if anyone else finds it useful and needs something to change or has a contribution to make.

# Author

- Tanja Álvarez, 2023
