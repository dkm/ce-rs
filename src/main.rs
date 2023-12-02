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

use clap::{Arg, ArgGroup, ArgMatches, Command};
use serde::Serialize;

use colored::*;

use regex::Regex;

use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Language {
    id: String,
    name: String,
    extensions: Vec<String>,
    monaco: String,
}

#[allow(dead_code)]
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

impl CompilerInfo {
    pub fn to_text(&self) -> String {
        format!(
            "\"{}\", id: {}, language: {}, type: {}, version: {}, ISA: {}",
            self.name, self.id, self.lang, self.compilerType, self.semver, self.instructionSet
        )
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct ShortLinkInfo {
    sessions: Vec<Session>,
    trees: Vec<Tree>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Tree {}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct ExecutorConfig {}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Output {}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Library {}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Tool {}

#[allow(dead_code)]
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

#[allow(non_snake_case)]
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

    pub fn new() -> Self {
        Filters {
            binary: false,
            binaryObject: false,
            execute: false,

            commentOnly: true,
            demangle: true,
            directives: true,
            intel: true,
            labels: true,
            libraryCode: true,
            trim: false,
            debugCalls: true,
        }
    }

    pub fn binary(mut self, v: bool) -> Self {
        self.binary = v;
        self
    }

    pub fn binary_object(mut self, v: bool) -> Self {
        self.binaryObject = v;
        self
    }

    pub fn comment_only(mut self, v: bool) -> Self {
        self.commentOnly = v;
        self
    }

    pub fn demangle(mut self, v: bool) -> Self {
        self.demangle = v;
        self
    }

    pub fn directives(mut self, v: bool) -> Self {
        self.directives = v;
        self
    }

    pub fn execute(mut self, v: bool) -> Self {
        self.execute = v;
        self
    }

    pub fn intel(mut self, v: bool) -> Self {
        self.intel = v;
        self
    }

    pub fn labels(mut self, v: bool) -> Self {
        self.labels = v;
        self
    }

    pub fn libraryCode(mut self, v: bool) -> Self {
        self.libraryCode = v;
        self
    }

    pub fn trim(mut self, v: bool) -> Self {
        self.trim = v;
        self
    }

    pub fn debugCalls(mut self, v: bool) -> Self {
        self.debugCalls = v;
        self
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone)]
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

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone)]
struct CompileOptions {
    userArguments: String,
    compilerOptions: OtherCompilerOptions,
    filters: Filters,
    tools: Vec<Tool>,
    libraries: Vec<Library>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
struct OtherCompilerOptions {
    skipAsm: bool,
    executorRequest: bool,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Download {}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct ToolResult {}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Label {}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Tag {
    line: i32,
    column: i32,
    text: String,
    severity: i32,
    file: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct SomeOutput(Vec<OutputItem>);

impl SomeOutput {
    pub fn to_text(&self) -> String {
        self.0
            .iter()
            .map(|x| x.text.clone())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct OutputItem {
    text: String,
    tag: Option<Tag>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct SourceLocation {
    file: Option<String>,
    line: i32,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct AsmOutput(Vec<AsmOutputItem>);

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct AsmOutputItem {
    text: String,
    source: Option<SourceLocation>,
    labels: Vec<Label>,
}

impl AsmOutput {
    pub fn to_text(&self) -> String {
        self.0
            .iter()
            .map(|x| x.text.clone())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct PopularArgument {
    description: String,
    timesused: i32,
}

#[allow(dead_code)]
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
    popularArguments: Option<HashMap<String, PopularArgument>>,
    execResult: Option<ExecutionResult>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct ExecutionResult {
    code: i32,
    okToCache: Option<bool>,
    timedOut: bool,
    stdout: SomeOutput,
    stderr: SomeOutput,
    truncated: Option<bool>,
    execTime: Option<String>,
    processExecutionResultTime: Option<f32>,
    didExecute: bool,
    buildResult: ExecBuildResult,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct ExecBuildResult {
    inputFilename: String,
    code: i32,
    okToCache: bool,
    timedOut: bool,
    stdout: SomeOutput,
    stderr: SomeOutput,
    truncated: bool,
    execTime: String,
    processExecutionResultTime: f32,
    downloads: Vec<Download>,
    executableFilename: String,
    compilationOptions: Vec<String>,
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

async fn compilers(base_url: &str) -> Result<Vec<CompilerInfo>, Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/compilers", base_url))
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
        .get(format!(
            "https://godbolt.org/api/shortlinkinfo/{}",
            shortlink
        ))
        .header("Accept", "application/json")
        .send()
        .await?;

    let resp = resp.json::<ShortLinkInfo>().await?;
    Ok(resp)
}

async fn compile(
    base_url: &str,
    compiler_id: &str,
    job: CompileJob,
) -> Result<CompileJobResult, Error> {
    let client = reqwest::Client::new();

    let resp = client
        .post(format!("{}/api/compiler/{}/compile", base_url, compiler_id))
        .header("Accept", "application/json")
        .json(&job)
        .send()
        .await?;

    let resp = resp.json::<CompileJobResult>().await?;
    Ok(resp)
}

async fn find_compilers(
    base_url: &str,
    name: Option<String>,
    language: Option<String>,
    isa: Option<String>,
) -> Option<Vec<CompilerInfo>> {
    if let Ok(all_compilers) = compilers(base_url).await {
        let after_name_filtered = match name {
            Some(n) => {
                let re = Regex::new(format!(r"(?i){}", n).as_str()).unwrap();
                all_compilers
                    .into_iter()
                    .filter(|x| re.captures(&x.name).is_some())
                    .collect::<Vec<CompilerInfo>>()
            }
            _ => all_compilers,
        };

        let after_lang_filtered = match language {
            Some(lang) => {
                // We use an exact match for lang
                let re = Regex::new(format!(r"(?i)^{}$", lang).as_str()).unwrap();
                after_name_filtered
                    .into_iter()
                    .filter(|x| re.captures(&x.lang).is_some())
                    .collect::<Vec<CompilerInfo>>()
            }
            _ => after_name_filtered,
        };

        let after_isa_filtered = match isa {
            Some(misa) => {
                // We use an exact match for ISA
                let re = Regex::new(format!(r"(?i)^{}$", misa).as_str()).unwrap();
                after_lang_filtered
                    .into_iter()
                    .filter(|x| re.captures(&x.instructionSet).is_some())
                    .collect::<Vec<CompilerInfo>>()
            }
            _ => after_lang_filtered,
        };

        return Some(after_isa_filtered);
    }
    None
}

async fn do_list_compilers(base_url: &str, matches: &ArgMatches) {
    let name = matches.get_one::<String>("name");
    let lang = matches.get_one::<String>("language");
    let isa = matches.get_one::<String>("isa");

    let maybe_compilers =
        find_compilers(base_url, name.cloned(), lang.cloned(), isa.cloned()).await;
    if let Some(compilers) = maybe_compilers {
        for c in compilers {
            println!("- {}", c.to_text());
        }
    } else {
        println!("No compiler found");
    }
}

async fn do_compile(base_url: &str, matches: &ArgMatches) {
    let is_summary = matches.get_one::<bool>("summary").unwrap();

    let mut filters_config = Filters::new()
        .binary(*matches.get_one("binary").unwrap())
        .binary_object(*matches.get_one("binary-object").unwrap())
        .execute(*matches.get_one("execute").unwrap());

    let mut stdout_f = match matches.get_one::<String>("stdout") {
        Some(ref s) if *s == "-" => Some(Box::new(std::io::stdout()) as Box<dyn std::io::Write>),
        Some(filename) => std::fs::File::create(filename)
            .map(|f| Some(Box::new(f) as Box<dyn std::io::Write>))
            .unwrap(),
        _ => None,
    };

    let mut stderr_f = match matches.get_one::<String>("stderr") {
        Some(ref s) if *s == "-" => Some(Box::new(std::io::stderr()) as Box<dyn std::io::Write>),
        Some(filename) => std::fs::File::create(filename)
            .map(|f| Some(Box::new(f) as Box<dyn std::io::Write>))
            .unwrap(),
        _ => None,
    };

    if let Some(filters) = matches.get_many::<String>("filters") {
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

    let source_data = if let Some(source_text) = matches.get_one::<String>("source") {
        source_text.clone()
    } else if let Some(source_file) = matches.get_one::<String>("source-file") {
        std::fs::read_to_string(source_file).expect("Unable to read file")
    } else {
        "no".to_string() // FIXME
    };

    let flags = if let Some(f) = matches.get_one::<String>("flags") {
        f.clone()
    } else {
        "".to_string()
    };

    let simple_job = CompileJob::build(&source_data, &flags, &filters_config);

    let compilers_id = if let Some(id) = matches.get_one::<String>("compiler-id") {
        vec![id.clone()]
    } else {
        let name = matches.get_one::<String>("compiler-name");
        let lang = matches.get_one::<String>("compiler-lang");
        let isa = matches.get_one::<String>("compiler-isa");

        find_compilers(base_url, name.cloned(), lang.cloned(), isa.cloned())
            .await
            .unwrap()
            .into_iter()
            .map(|cinfo| cinfo.id)
            .collect::<Vec<String>>()
    };

    for compiler_id in compilers_id {
        let compile_ret1 = compile(base_url, &compiler_id, simple_job.clone()).await;

        let ret1 = compile_ret1.unwrap();

        if let Some(ref mut f) = &mut stdout_f {
            f.write_all(ret1.stdout.to_text().as_bytes()).unwrap();
        }

        if let Some(ref mut f) = &mut stderr_f {
            f.write_all(ret1.stderr.to_text().as_bytes()).unwrap();
        }

        if !is_summary {
            println!("{}", ret1.asm.to_text());
        } else {
            println!(
                "{} Compilation {}({})",
                compiler_id,
                (if ret1.code == 0 {
                    "✔".green()
                } else {
                    "✗".red()
                }),
                ret1.code
            );
        }

        if let Some(exec_result) = ret1.execResult {
            if !is_summary {
                println!("Execution:\n{}", exec_result.stdout.to_text());
            } else {
                println!(
                    "{} Execution {}({})",
                    compiler_id,
                    (if exec_result.code == 0 {
                        "✔".green()
                    } else {
                        "✗".red()
                    }),
                    exec_result.code
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
enum OutputConfig {
    Disable,
    ToFile(String),
    ToStdout,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("ce-rs")
        .version("0.1")
        .author("Marc Poulhiès <dkm@kataplop.net>")
        .about("Play with compiler-explorer")
        .arg(
            Arg::new("base-url")
                .long("base-url")
                .default_value("https://godbolt.org"),
        )
        .subcommand(
            Command::new("list-compilers")
                .arg(Arg::new("all").action(clap::ArgAction::SetTrue).long("all"))
                .arg(Arg::new("name").long("name"))
                .arg(Arg::new("isa").long("instruction-set"))
                .arg(Arg::new("language").long("language")),
        )
        .subcommand(
            Command::new("compile")
                .arg(
                    Arg::new("source")
                        .conflicts_with("source-file")
                        .long("source"),
                )
                .arg(
                    Arg::new("binary")
                        .action(clap::ArgAction::SetTrue)
                        .long("binary")
                        .conflicts_with("binary-object")
                        .conflicts_with("execute"),
                )
                .arg(
                    Arg::new("binary-object")
                        .action(clap::ArgAction::SetTrue)
                        .long("binary-object")
                        .conflicts_with("execute")
                        .conflicts_with("binary"),
                )
                .arg(
                    Arg::new("execute")
                        .action(clap::ArgAction::SetTrue)
                        .long("execute")
                        .conflicts_with("binary-object")
                        .conflicts_with("binary"),
                )
                .arg(
                    Arg::new("summary")
                        .long("summary")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("source-file")
                        .long("source-file")
                        .conflicts_with("source"),
                )
                .group(
                    ArgGroup::new("source-group")
                        .args(["source", "source-file"])
                        .required(true)
                        .multiple(false),
                )
                .arg(Arg::new("compiler-id").long("id"))
                .arg(
                    Arg::new("compiler-name")
                        .long("name")
                        .conflicts_with("compiler-id"),
                )
                .arg(
                    Arg::new("compiler-lang")
                        .long("language")
                        .conflicts_with("compiler-id"),
                )
                .arg(
                    Arg::new("compiler-isa")
                        .long("instruction-set")
                        .conflicts_with("compiler-id"),
                )
                .arg(Arg::new("flags").allow_hyphen_values(true).long("flags"))
                .arg(
                    Arg::new("stdout")
                        .long("stdout")
                        .help("Write stdout to given file (stdout if -)"),
                )
                .arg(
                    Arg::new("stderr")
                        .long("stderr")
                        .help("Write stderr to given file (stdout if -)"),
                )
                .arg(
                    Arg::new("filters")
                        .long("filters")
                        .short('f')
                        .value_delimiter(','),
                ),
        )
        .get_matches();

    // let stderr_config = match matches.get_one::<String>("stderr") {
    //     Some("-".to_string(value)) => OutputConfig::Disable,
    //     Some(filename) => OutputConfig::Disable,
    //     _ => OutputConfig::Disable,
    // };

    let base_url = matches
        .get_one::<String>("base-url")
        .expect("can't be missing");
    match matches.subcommand() {
        Some(("compile", sub_matches)) => do_compile(&base_url, sub_matches).await,
        Some(("list-compilers", sub_matches)) => do_list_compilers(&base_url, sub_matches).await,
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
    //     println!("stdout: {}", ret1.stdout.to_text());
    //     println!("stderr: {}", ret1.stderr.to_text());
    //     println!("asm: {}", ret1.asm.to_text());

    Ok(())
}
