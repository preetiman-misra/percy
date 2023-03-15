# Percy

Percy is an application that allows it's users to recursively
index a given `data` directory into an `index.json` file.

# Goal

It's goal is to ultimately evolve into a personal search engine,
allowing for the creation of knowledge databases that are:

- searchable
- scalable
- deployable (web)

# Usage

You can clone the GitHub Repo and walk through the code
on your own machine by doing the following:

```zsh
git clone https://github.com/preetiman-misra/percy.git
cd percy
cargo run
```

You can modify the directory structure of the `data` directory and percy will
index the files inside accordingly.

Indexing:

```
data/
  nested_one/
    content.md
    other.txt
  hello.html
  wow.txt
```

Will result in the following `index.json` file:

```json
{
  "notes": [
    {
      "path": "data_copy/nested_one/content.md",
      "content": "## Markdown content\n",
      "file_type": "md"
    },
    {
      "path": "data_copy/nested_one/other.txt",
      "content": "text content\n",
      "file_type": "txt"
    },
    {
      "path": "data_copy/hello.html",
      "content": "<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"UTF-8\" />\n    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    <title>Hello</title>\n  </head>\n  <body>\n    <h1>Some HTML</h1>\n  </body>\n</html>\n",
      "file_type": "html"
    },
    {
      "path": "data_copy/wow.txt",
      "content": "WOWOWOW\n",
      "file_type": "txt"
    }
  ]
}
```

# Documentation

The documentation can be perused by using the following command in the `percy` directory:

```zsh
cargo doc --no-deps --open
```
