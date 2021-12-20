# sitegen

A static site generator.

Install:
  $ git clone https://github.com/tedbauer/sitegen
  $ cd sitegen
  $ cargo build
  $ alias sitegen="$(PWD)/target/debug/sitegen"

Create a new site:
    $ sitegen --new -d example_site # create site at $(PWD)/example_site
    $ cd example_site
    $ echo "example webpage 1 $$ x+5 $$" > site/page1.html
    $ echo "example webpage 2 $$ \frac{x}{y} $$" > site/page2.html
    $ sitegen

Place HTML files in `site/`, and a table of contents will be generated in `index.html`.
You can write math expressions like `$$ x + 5 $$`, and they'll be rendered with [KaTeX](https://katex.org/).