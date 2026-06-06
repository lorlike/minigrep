
# Introduction

A mini rust demo.

```bash
> minigrep --help 

minigrep by lorlike

Usage: minigrep [OPTIONS] <QUERY> <FILENAME>

Arguments:
  <QUERY>     the string for query
  <FILENAME>  the filename to query

Options:
  -c, --case-insensitive  match insensitively
  -h, --help              Print help
  -V, --version           Print version


```

## sensitively match

```bash
> minigrep to poem.txt

Are you nobody, too?
How dreary to be somebody!
```

## insensitively match

```bash
> minigrep to poem.txt -c

Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```
