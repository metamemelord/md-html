#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]

extern crate clap;
extern crate maud;
extern crate pulldown_cmark;

use clap::{clap_app, crate_version};
use maud::html;
use pulldown_cmark::{html::push_html, Event, Parser};
use std::io::Write;

fn wrap_html(s: &str, css: Option<&str>) -> String {
    let res = html! {
      (maud::DOCTYPE)
        html {
          head {
            meta charset="utf-8";
            @if let Some(s) = css {
              link rel="stylesheet" type="text/css" href=(s);
            }
          }
          body {
            (maud::PreEscaped(s))
          }
        }
    };
    res.into_string()
}

fn main() {
    let clap = clap_app!(
      markdown_html =>
      (version:crate_version!())
      (author:"Gaurav Saini")
      (about:"Simple markdown renderer!")
      (@arg input: +required "Set the input file")
      (@arg output: -o --output +takes_value "HTML output file")
      (@arg wrap: -w "Wrap in html")
      (@arg css: --css +takes_value "Link to css")
    )
    .get_matches();

    // let input_file = clap.value_of("input");
    // println!("input_file: {:?}", input_file);
    let read_file = std::fs::read_to_string(clap.value_of("input").unwrap())
        .expect("Could not read the input file");
    let mut res = String::new();
    let ps = Parser::new(&read_file);
    push_html(&mut res, ps.into_iter());

    if clap.is_present("wrap") {
        res = wrap_html(&res, clap.value_of("css"));
    }

    println!("{}", &res);

    if clap.is_present("output") {
        let mut write_file = std::fs::File::create(clap.value_of("output").unwrap()).unwrap();

        match write_file.write_all(res.as_bytes()) {
            Ok(_) => println!(
                "Output written to file: {}",
                clap.value_of("output").unwrap()
            ),
            Err(err) => println!("{:?}", err),
        };
    }
}
