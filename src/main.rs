extern crate syn;
extern crate clap;

use std::fmt;
use std::fs::File;
use std::io::Read;

use clap::{Arg, App};
use syn::{visit, Expr, ExprCall, UseTree, ItemUse};

#[derive(Debug, Copy, Clone, Default)]
pub struct Permissions {
  unsafe_permission: bool,
  fs_permission: bool,
  net_permission: bool,
  io_permission: bool,
  process_permission: bool,
  thread_permission: bool,
}

impl Permissions {
  fn check (&mut self, token: &str) {
    match token {
      "fs" => {
        self.fs_permission = true;
      }
      "net" => {
        self.net_permission = true;
      }
      "io" => {
        self.io_permission = true;
      }
      "process" => {
        self.process_permission = true;
      }
      "thread" => {
        self.thread_permission = true;
      }
      _ => {}
    }
  }
}

impl<'ast> visit::Visit<'ast> for Permissions {
  fn visit_expr_call(&mut self, i: &ExprCall) {
    match &*i.func {
      Expr::Path(p) => {
        let mut iter = p.path.segments.iter();

        if let Some(stduse) = iter.next() {
          if  let Some(stdperm) = iter.next() {
            if stduse.ident.to_string() == "std" {
              self.check(stdperm.ident.to_string().as_ref());
            }
          }
        }
      }
      _ => {}
    }
  }

  fn visit_item_use(&mut self, i: &ItemUse) {
    // TODO: Process Groups
    match &i.tree {
      UseTree::Path(path) => {
        if path.ident.to_string() == "std" {
          match &*path.tree {
            UseTree::Path(l2) => {
              self.check(l2.ident.to_string().as_ref());
            }
            UseTree::Name(l2) => {
              self.check(l2.ident.to_string().as_ref());
            }
            _ => {}
          }
        }
      }
      _ => {}
    }

    visit::visit_item_use(self, i);
  }
}

impl fmt::Display for Permissions {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "unsafe: {}", self.unsafe_permission);
    writeln!(f, "fs: {}", self.fs_permission)?;
    writeln!(f, "net: {}", self.net_permission)?;
    writeln!(f, "io: {}", self.io_permission)?;
    writeln!(f, "process: {}", self.process_permission)?;
    writeln!(f, "thread: {}", self.thread_permission)
  }
}

fn main() {
  let matches = App::new("cargo-permissions")
    .about("Prints used permissions in a Rust file.")
    .arg(Arg::with_name("files")
         .required(true)
         .takes_value(true)
         .multiple(true)
         .help("Files to process")
    )
    .get_matches();

  let tracker = &mut Permissions::default();

  if let Some(v) = matches.values_of("files") {
    for filename in v {
      println!("Processing file {}", filename);
      let mut file = File::open(filename).expect("Unable to open file");

      let mut src = String::new();
      file.read_to_string(&mut src).expect("Unable to read file");

      let syntax = syn::parse_file(&src).expect("Unable to parse file");
      visit::visit_file(tracker, &syntax);
    }
  }
  println!("{}", tracker);
}
