/*
 * This file is part of the ce-rs (https://github.com/dkm/ce-rs)
 * Copyright (c) 2023 Marc Poulhiès <dkm@kataplop.net>.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use serde::Serialize;
use clap::{Arg, Command, ArgMatches};

use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Language {
    id: String,
    name: String,
    extensions: Vec<String>,
    monaco: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct CompilerInfo {
    id: String,
    name: String,
    lang: String,
    compilerType: String,
    semver: String,
    instructionSet: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Session {
    id: u8,
    language: String,
    source: String,
    conformanceview: bool,
    compilers: Vec<CompilerConfig>,
    executors: Vec<ExecutorConfig>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct ShortLinkInfo {
    sessions: Vec<Session>,
    trees: Vec<Tree>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Tree {
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct ExecutorConfig {
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct CompilerConfig {
    _internalid: u8,
    id: String,
    options: String,
    filters: Filters,
    libs: Vec<Library>,
    specialoutputs: Vec<Output>,
    tools: Vec<Tool>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Output {}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct Library {}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct Tool {}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Filters {
    binary: bool,
    binaryObject: bool,
    commentOnly: bool,
    demangle: bool,
    directives: bool,
    execute: bool,
    intel: bool,
    labels: bool,
    libraryCode: bool,
    trim: bool,
    debugCalls: bool,
}

impl Filters {
    pub fn all_disabled() -> Self {
        Filters {
            binary: false,
            binaryObject: false,
            commentOnly: false,
            demangle: false,
            directives: false,
            execute: false,
            intel: false,
            labels: false,
            libraryCode: false,
            trim: false,
            debugCalls: false,
        }
    }
    pub fn default() -> Self {
        Filters {
            binary: false,
            binaryObject: false,
            commentOnly: true,
            demangle: true,
            directives: true,
            execute: false,
            intel: true,
            labels: true,
            libraryCode: true,
            trim: false,
            debugCalls: true,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct CompileJob {
    source: String,
    options: CompileOptions,
    lang: Option<String>,
    allowStoreCodeDebug: bool,
}

impl CompileJob {
    pub fn build(source: &str, compiler_option: &str, filters: &Filters) -> Self {
        CompileJob {
            source: source.to_string(),
            options: CompileOptions {
                userArguments: compiler_option.to_string(),
                compilerOptions: OtherCompilerOptions {
                    skipAsm: false,
                    executorRequest: false,
                },
                filters: (*filters).clone(),
                tools: Vec::new(),
                libraries: Vec::new(),
            },
            lang: None,
            allowStoreCodeDebug: true,
        }
    }

    pub fn build_simple(source: &str, compiler_option: &str) -> Self {
        CompileJob {
            source: source.to_string(),
            options: CompileOptions {
                userArguments: compiler_option.to_string(),
                compilerOptions: OtherCompilerOptions {
                    skipAsm: false,
                    executorRequest: false,
                },
                filters: Filters {
                    binary: false,
                    binaryObject: false,
                    commentOnly: false,
                    demangle: false,
                    directives: false,
                    execute: false,
                    intel: false,
                    labels: false,
                    libraryCode: false,
                    trim: false,
                    debugCalls: false,
                },
                tools: Vec::new(),
                libraries: Vec::new(),
            },
            lang: None,
            allowStoreCodeDebug: true,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct CompileOptions {
    userArguments: String,
    compilerOptions: OtherCompilerOptions,
    filters: Filters,
    tools: Vec<Tool>,
    libraries: Vec<Library>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct OtherCompilerOptions {
    skipAsm: bool,
    executorRequest: bool,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Download { }

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct ToolResult { }

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Label { }

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Tag {
    line: i32,
    column: i32,
    text: String,
    severity: i32,
    file: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct SomeOutput(Vec<OutputItem>);

impl SomeOutput {
    pub fn toText(&self) -> String {
        self.0.iter().map(|x| {
            println!("Text: {}", x.text);
            x.text.clone()
        }).collect::<Vec<String>>().join("\n")
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct OutputItem {
    text: String,
    tag: Option<Tag>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct SourceLocation {
    file: Option<String>,
    line: i32,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct AsmOutput(Vec<AsmOutputItem>);

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct AsmOutputItem {
    text: String,
    source: Option<SourceLocation>,
    labels: Vec<Label>,
}


impl AsmOutput {
    pub fn toText(&self) -> String {
        self.0.iter().map(|x| {
            println!("Text: {}", x.text);
            x.text.clone()
        }).collect::<Vec<String>>().join("\n")
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct PopularArgument {
    description: String,
    timesused: i32,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct CompileJobResult {
    inputFilename: String,
    code: i32,
    okToCache: bool,
    timedOut: bool,
    stdout: SomeOutput,
    stderr: SomeOutput,
    truncated: bool,
    execTime: String, // why not integer?
    processExecutionResultTime: f32,
    compilationOptions: Vec<String>,
    downloads: Vec<Download>,
    tools: Vec<ToolResult>,
    asm: AsmOutput,
    labelDefinitions: HashMap<String, i32>,
    parsingTime: String, // why not integer?
    filteredCount: i32,
    popularArguments: HashMap<String, PopularArgument>,
}

#[derive(Debug, Error)]
enum Error {
    #[error("Reqwest error")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },

    #[error("Serde error")]
    SerdeError {
        #[from]
        source: serde_json::Error,
    },
}

async fn languages() -> Result<Vec<Language>, Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://godbolt.org/api/languages")
        .header("Accept", "application/json")
        .send()
        .await?;

    let resp = resp.json::<Vec<Language>>().await?;
    Ok(resp)
}

async fn compilers() -> Result<Vec<CompilerInfo>, Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://godbolt.org/api/compilers")
        .header("Accept", "application/json")
        .send()
        .await?;

    let resp = resp.json::<Vec<CompilerInfo>>().await?;
    Ok(resp)
}

async fn compilers_id(id: u8) -> Result<Vec<CompilerInfo>, Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://godbolt.org/api/compilers/{}", id))
        .header("Accept", "application/json")
        .send()
        .await?;

    let resp = resp.json::<Vec<CompilerInfo>>().await?;
    Ok(resp)
}

async fn shortlinkinfo(shortlink: &str) -> Result<ShortLinkInfo, Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://godbolt.org/api/shortlinkinfo/{}", shortlink))
        .header("Accept", "application/json")
        .send()
        .await?;

    let resp = resp.json::<ShortLinkInfo>().await?;
    Ok(resp)
}

async fn compile(compiler_id: &str, job: CompileJob) -> Result<CompileJobResult, Error> {
    let client = reqwest::Client::new();

    let resp = client
        .post(format!("https://godbolt.org/api/compiler/{}/compile", compiler_id))
        .header("Accept", "application/json")
        .json(&job)
        .send()
        .await?;

    let resp = resp.json::<CompileJobResult>().await?;
    Ok(resp)
}

async fn do_compile(matches : &ArgMatches) {
    let mut filters_config = Filters::default();

    if let Some(filters) = matches.get_many::<String> ("filters") {
    //let enabled_filters: Vec<_> = matches.get_many::<String> ("filters").unwrap().collect();
        filters_config = Filters::all_disabled();
        let enabled_filters: Vec<_> = filters.collect();
        for f in enabled_filters {
            match f.as_str() {
                "binary" => filters_config.binary = true,
                "binaryObject" => filters_config.binaryObject = true,
                "commentOnly" => filters_config.commentOnly = true,
                "demangle" => filters_config.demangle = true,
                "directives" => filters_config.directives = true,
                "execute" => filters_config.execute = true,
                "intel" => filters_config.intel = true,
                "labels" => filters_config.labels = true,
                "libraryCode" => filters_config.libraryCode = true,
                "trim" => filters_config.trim = true,
                "debugCalls" => filters_config.debugCalls = true,
                &_ => println!("Unknown filter: {}", f),
            }
        }
    }

    let compiler_id = matches.get_one::<String>("compiler-id").expect("Missing compiler-id");
    let source_code = matches.get_one::<String>("source").expect("Missing source code");

    let simple_job = CompileJob::build(source_code, "", &filters_config);
    let compile_ret1 = compile(compiler_id, simple_job).await;

    let ret1 = compile_ret1.unwrap();
    println!("stdout: {}", ret1.stdout.toText());
    println!("stderr: {}", ret1.stderr.toText());
    println!("asm: {}", ret1.asm.toText());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("ce-rs")
        .version("0.1")
        .author("Marc Poulhiès <dkm@kataplop.net>")
        .about("Play with compiler-explorer")
        .arg(Arg::new("base-url")
            .long("base-url")
            .default_value("https://godbolt.org/api/"))
        .subcommand(
            Command::new("compile")
                .arg(Arg::new("source")
                    .long("source"))
                .arg(Arg::new("compiler-id")
                    .long("compiler-id")
                    .short('c'))
                .arg(Arg::new("flags")
                    .long("flags"))
                .arg(Arg::new("filters")
                     .long("filters")
                     .short('f')
                     .value_delimiter(',')))
        .get_matches();

    match matches.subcommand() {
        Some(("compile", sub_matches)) => do_compile(sub_matches).await,
        _ => println!("Woops"),
    }

//     let lang = languages().await;
//     // println!("Languages: {:?}", lang);

//     let comps = compilers().await;
//     // println!("Compilers: {:?}", comps);

//     let linkinfo = shortlinkinfo(&"hMh7fcbs1").await;
//     ///println!("linkinfo: {:?}", linkinfo);

//     let linkinfo = shortlinkinfo(&"s6vGq7359").await;
//     // println!("linkinfo: {:?}", linkinfo); //

//     let simple_job = CompileJob::build_simple("int main() {return 0;}", "");
//     let compile_ret1 = compile("cg412", simple_job).await;
// //    println!("compile job result: {:?}", compile_ret1);
//     let ret1 = compile_ret1.unwrap();
//     println!("stdout: {}", ret1.stdout.toText());
//     println!("stderr: {}", ret1.stderr.toText());
//     println!("asm: {}", ret1.asm.toText());

    Ok(())
}
