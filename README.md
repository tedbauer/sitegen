# sitegen

A static site generator.

Create a new site:
```
git clone https://github.com/tedbauer/sitegen
cd sitegen

# Create a new site called 'test-site'.
cargo run -- --new --name test-site
```

Generated site structure:
```
site-name/
  site/
	  page1.md
		...
	index.html
```

Place markdown files in `site-name`, and a table of contents will be generated in `index.html`.
You can write math expressions like `$$ x + 5 $$`, and they'll be rendered with [KaTeX](https://katex.org/).