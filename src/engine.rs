use std::path::PathBuf;
use std::path::MAIN_SEPARATOR;
use crate::nodes::ScriptType;
use std::fs::File;
use std::env;
use std::process;
use std::io::Read;
use tokesies::*;
use tokesies::Token;
use crate::lex::SiliconFilter;
use crate::lex::CommentType;

pub struct Program {
    root_dir : PathBuf
}

impl Program {

    pub fn find(&self) {

    }
}

pub fn run(path : PathBuf) {

    let root : Program = Program {
        root_dir : root_directory(&path)
    };

    let mut  main_class_file : File = open_script(&path);
    let mut contents;
    main_class_file.read_to_string(contents);

    let tokens = FilteredTokenizer::new(SiliconFilter {
        comment_mode: CommentType::Off
    }, contents);
    let tokens : Vec<Token> = tokens.collect();

    for i in tokens.len()  {
        let term = (tokens.get(i) as Token).term();
        print!("{}", term)
    }

    println!("{:?}", path);
    println!("{:?}", root.root_dir);

    return;
}

fn root_directory(mut path : &PathBuf) -> PathBuf {

    let path = path.as_path();
    let path = path.parent().unwrap();
    let root = path.to_path_buf();
    return root

}

fn class_path(path : &PathBuf, import : String) -> PathBuf {
    let mut split = import.split(".");
    let components : Vec<&str> = split.collect();

    let mut class_path = path.clone();

    for component in components {
        class_path.push(component)
    }

    class_path.set_extension(".silicon");

    return class_path;
}

fn open_script(path : &PathBuf) -> File {
    let mut path = path.clone();
    path.set_extension("silicon");

    let mut file : File;
    match File::open(&path) {
        Ok(T) => file = T,
        Err(E) => {
            println!("File {:?} is not a valid script file!", path.to_str());
            process::exit(1)
        }
    }
    return file
}