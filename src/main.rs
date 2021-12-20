use std::fs::File;
use std::fs::{self};
use std::io::Write;
use std::path::PathBuf;
extern crate argparse;
<<<<<<< HEAD
use anyhow::anyhow;
use anyhow::Result;
=======
>>>>>>> 00d82f61abfb453c17e229b7de357a80ae512221
use argparse::{ArgumentParser, Store, StoreTrue};

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

<<<<<<< HEAD
struct Options {
    new_site: bool,
    site_root_dir: PathBuf,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            new_site: false,
            site_root_dir: PathBuf::from("."),
        }
    }
}

fn build_pages(options: &Options) -> Result<()> {
    let pages: Result<Vec<(String, String)>> = fs::read_dir(options.site_root_dir.join("site"))?
        .map(|entry| {
            let entry = entry?;
            Ok((
                entry
                    .file_name()
                    .to_str()
                    .ok_or(anyhow!("filename not valid utf8"))?
                    .to_string(),
                fs::read_to_string(&entry.path())?,
            ))
        })
        .collect();

    fs::create_dir_all(options.site_root_dir.join("pages"))?;
    for (page_name, page_contents) in pages? {
        let mut page_file = File::create(options.site_root_dir.join("pages").join(&page_name))?;
        page_file.write_all(wrap_in_html(&page_contents).as_bytes())?;
    }
    Ok(())
}

fn build_index(options: &Options) -> Result<()> {
    let table_entries: Result<String> =
        fs::read_dir(options.site_root_dir.join("site"))?.try_fold(String::new(), |acc, entry| {
            let filename = entry?.file_name();
            let filename_str = filename
                .to_str()
                .ok_or(anyhow!("filename not valid utf8"))?;
            let list_entry = format!(
                r#"<li><a href="pages/{}">{}</a></li>"#,
                filename_str, filename_str
            );
            println!("acc: {}", acc);
            Ok(acc + &list_entry)
        });
    let table_of_contents = format!("<ul>\n{}</ul>", table_entries?);

    let mut file = File::create(options.site_root_dir.join("index.html"))?;
    Ok(file.write_all(wrap_in_html(&table_of_contents).as_bytes())?)
}

fn build_site(options: &Options) -> Result<()> {
    build_index(options)?;
    build_pages(options)?;
    println!(
        "Built '{}'.",
        options
            .site_root_dir
            .to_str()
            .ok_or(anyhow!("site root dir is not valid unicode"))?,
    );
    Ok(())
}

fn create_new_site(options: &Options) -> Result<()> {
    fs::create_dir(&options.site_root_dir)?;
    fs::create_dir(options.site_root_dir.join("site"))?;

    let mut index = File::create(options.site_root_dir.join("index.html"))?;
    index.write_all(b"Welcome to my new site created with sitegen")?;

    println!(
        "Generated new site at '{}'. ",
        options
            .site_root_dir
            .to_str()
            .ok_or(anyhow!("site root dir is not valid unicode"))?
    );
    Ok(())
}

fn get_options() -> Result<Options> {
    let mut options = Options::default();
    let mut parser = ArgumentParser::new();

    parser.refer(&mut options.new_site).add_option(
        &["--new", "-n"],
        StoreTrue,
        "Generate new site",
    );

    parser.refer(&mut options.site_root_dir).add_option(
        &["--dir", "-d"],
        Store,
        "The root directory of the site",
    );

    parser.parse_args_or_exit();
    drop(parser);

    Ok(options)
}

fn main() -> std::io::Result<()> {
    let options = get_options().unwrap();

    if options.new_site {
        create_new_site(&options).unwrap();
    } else {
        build_site(&options).unwrap();
    }

=======
fn main() -> std::io::Result<()> {
    let mut new_site = false;
    let mut build = false;
    let mut site_name = "my_site".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut new_site)
            .add_option(&["-n", "--new"], StoreTrue, "Generate new site");

        ap.refer(&mut build)
            .add_option(&["-b", "--build"], StoreTrue, "Build site");

        ap.refer(&mut site_name)
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
        for entry in fs::read_dir(format!("{}/site", site_name))? {
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

        let mut file = File::create(format!("{}/index.html", site_name))?;
        file.write_all(wrap_in_html(&table_of_contents).as_bytes())?;
        fs::create_dir(format!("{}/pages", site_name))?;
        for (page_name, page_contents) in pages {
            let mut page_file = File::create(format!(
                "{}/pages/{}",
                site_name,
                str::replace(&page_name, "md", "html")
            ))?;
            page_file.write_all(wrap_in_html(&page_contents).as_bytes())?;
        }
    }
>>>>>>> 00d82f61abfb453c17e229b7de357a80ae512221
    Ok(())
}
