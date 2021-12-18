use std::fs::File;
use std::fs::{self};
use std::io::Write;
extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

fn wrap_in_html(body: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<!-- KaTeX requires the use of the HTML5 doctype. Without it, KaTeX may not render properly -->
<html>
  <head>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.15.1/dist/katex.min.css" integrity="sha384-R4558gYOUz8mP9YWpZJjofhk+zx0AS11p36HnD2ZKj/6JR5z27gSSULCNHIRReVs" crossorigin="anonymous">

    <!-- The loading of KaTeX is deferred to speed up page rendering -->
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.15.1/dist/katex.min.js" integrity="sha384-z1fJDqw8ZApjGO3/unPWUPsIymfsJmyrDVWC8Tv/a1HeOtGmkwNd/7xUS0Xcnvsx" crossorigin="anonymous"></script>

    <!-- To automatically render math in text elements, include the auto-render extension: -->
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.15.1/dist/contrib/auto-render.min.js" integrity="sha384-+XBljXPPiv+OzfbB3cVmLHf4hdUFHlWNZN5spNQ7rmHTXpd7WvJum6fIACpNNfIR" crossorigin="anonymous"
        onload="renderMathInElement(document.body);"></script>
  </head>
	{}
</html>"#,
        body
    )
}

fn main() -> std::io::Result<()> {
	let mut new_site = false;
	let mut build = false;
	let mut site_name = "my_site".to_string();
	{
		let mut ap = ArgumentParser::new();
		ap
			.refer(&mut new_site)
			.add_option(&["-n", "--new"], StoreTrue, "Generate new site");

		ap
			.refer(&mut build)
			.add_option(&["-b", "--build"], StoreTrue, "Build site");
		
		ap
			.refer(&mut site_name)
			.add_option(&["--name"], Store, "The name of the site");

		ap.parse_args_or_exit();
	}
	if new_site {
    fs::create_dir(format!("{}", site_name))?;
    fs::create_dir(format!("{}/site", site_name))?;
    let mut index = File::create(format!("{}/index.html", site_name))?;
    index.write_all(b"Welcome to my new site created with sitegen")?;
		println!("Generated new site '{}'. ", site_name);
	}
	if build {
    let mut table_of_contents = "<ul>\n".to_owned();
    let mut pages: Vec<(String, String)> = Vec::new();
    for entry in fs::read_dir("../site")? {
        let entry = entry?;
        table_of_contents.push_str("<li>");
        table_of_contents.push_str(&format!(
            r#"<a href="pages/{}">"#,
            str::replace(&entry.file_name().into_string().unwrap(), "md", "html")
        ));
        table_of_contents.push_str(&entry.file_name().into_string().unwrap());
        pages.push((
            entry.file_name().into_string().unwrap(),
            fs::read_to_string(&entry.path()).unwrap(),
        ));
        table_of_contents.push_str("</a>");
        table_of_contents.push_str("</li>");
        table_of_contents.push_str("\n");
    }
    table_of_contents.push_str("</ul>");

    let mut file = File::create("../index.html")?;
    file.write_all(wrap_in_html(&table_of_contents).as_bytes())?;
    fs::create_dir("../pages")?;
    for (page_name, page_contents) in pages {
        let mut page_file = File::create(format!(
            "../pages/{}",
            str::replace(&page_name, "md", "html")
        ))?;
        page_file.write_all(wrap_in_html(&page_contents).as_bytes())?;
    }
	}
    Ok(())
}
