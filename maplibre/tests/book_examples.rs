//! The book's example pages and the styles their maps load: every style parses as a style,
//! every page is reachable from the book's table of contents, and every page that embeds a
//! map embeds a style that exists.

use std::{fs, path::PathBuf};

use maplibre::style::Style;

fn examples() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../docs/src/examples")
}

#[cfg(test)]
fn read(path: &std::path::Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

#[test]
fn every_example_style_parses() {
    let styles = examples().join("demo/styles");
    let mut count = 0;
    for entry in fs::read_dir(&styles).expect("styles directory") {
        let path = entry.expect("style entry").path();
        if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            serde_json::from_str::<Style>(&read(&path))
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            count += 1;
        }
    }
    assert!(count > 0, "the book has example styles");
}

#[test]
fn every_example_page_is_in_the_table_of_contents_and_its_style_exists() {
    let summary = read(&examples().join("../SUMMARY-book.md"));
    for entry in fs::read_dir(examples()).expect("examples directory") {
        let path = entry.expect("example entry").path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !name.ends_with(".md") {
            continue;
        }
        assert!(
            summary.contains(&format!("examples/{name}")),
            "{name} is not linked from SUMMARY-book.md"
        );
        let page = read(&path);
        for style in page.split("?style=./styles/").skip(1) {
            let file = style.split(['"', '&']).next().unwrap_or_default();
            assert!(
                examples().join("demo/styles").join(file).is_file(),
                "{name} embeds the missing style {file}"
            );
        }
    }
}
