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
use colored::*;
use regex::Regex;
use thiserror::Error;
mod types;
use types::*;
use version_compare::{compare, compare_to, Cmp, Version};

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

    #[error("Internal Error")]
    InternalError,
}

struct Session {
    base_url: String,
    client: reqwest::Client,
    // all_compilers: Option<Vec<CompilerInfo>>,
    // all_compilers_full: bool,
}

async fn languages(session: &Session) -> Result<Vec<Language>, Error> {
    let resp = session.client
        .get(format!("{}/api/languages", session.base_url))
        .header("Accept", "application/json")
        .send()
        .await?;

    let resp = resp.json::<Vec<Language>>().await?;
    Ok(resp)
}

async fn compilers(session: &Session, all_fields: bool) -> Result<Vec<CompilerInfo>, Error> {
    let client = reqwest::Client::new();

    let params = [("fields", (if all_fields { "all" } else { "no" }))];

    let url = if all_fields {
        reqwest::Url::parse_with_params(&format!("{}/api/compilers", session.base_url), &params)
            .unwrap()
    } else {
        reqwest::Url::parse(&format!("{}/api/compilers", session.base_url)).unwrap()
    };

    let resp = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await?;

    let resp = resp.json::<Vec<CompilerInfo>>().await?;
    Ok(resp)
}

async fn compilers_id(session: &Session, id: u8) -> Result<Vec<CompilerInfo>, Error> {
    let resp = session.client
        .get(format!("{}/api/compilers/{}", session.base_url, id))
        .header("Accept", "application/json")
        .send()
        .await?;

    let resp = resp.json::<Vec<CompilerInfo>>().await?;
    Ok(resp)
}

async fn shortlinkinfo(session: &Session, shortlink: &str) -> Result<ShortLinkInfo, Error> {
    let resp = session.client
        .get(format!(
            "{}/api/shortlinkinfo/{}",
            session.base_url, shortlink
        ))
        .header("Accept", "application/json")
        .send()
        .await?;

    let resp = resp.json::<ShortLinkInfo>().await?;
    Ok(resp)
}

async fn compile(
    session: &Session,
    compiler_id: &str,
    job: CompileJob,
) -> Result<CompileJobResult, Error> {

    let resp = session.client
        .post(format!(
            "{}/api/compiler/{}/compile",
            session.base_url, compiler_id
        ))
        .header("Accept", "application/json")
        .json(&job)
        .send()
        .await?;

    let resp = resp.json::<CompileJobResult>().await?;
    Ok(resp)
}

async fn get_compiler_info(session: &Session, compiler_id: &str) -> Option<CompilerInfo> {
    if let Ok(all_compilers) = compilers(session, true).await {
        let all = all_compilers
            .into_iter()
            .filter(|x| x.id == compiler_id)
            .collect::<Vec<CompilerInfo>>();
        return Some(all[0].clone());
    }
    None
}

async fn find_compilers(
    session: &Session,
    all_fields: bool,
    name: Option<String>,
    language: Option<String>,
    isa: Option<String>,
    version_min: Option<String>,
    version_max: Option<String>,
) -> Option<Vec<CompilerInfo>> {
    if let Ok(all_compilers) = compilers(session, all_fields).await {
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

        let after_version_min_filtered = match version_min {
            Some(vmin) => {
                let vmin = Version::from(&vmin).unwrap();
                after_isa_filtered
                    .into_iter()
                    .filter(|x| {
                        let vcur = Version::from(&x.semver);
                        vcur.is_some_and(|cur| cur >= vmin)
                    })
                    .collect::<Vec<CompilerInfo>>()
            }
            _ => after_isa_filtered,
        };

        let after_version_max_filtered = match version_max {
            Some(vmax) => {
                let vmax = Version::from(&vmax).unwrap();
                after_version_min_filtered
                    .into_iter()
                    .filter(|x| {
                        let vcur = Version::from(&x.semver);
                        vcur.is_some_and(|cur| cur <= vmax)
                    })
                    .collect::<Vec<CompilerInfo>>()
            }
            _ => after_version_min_filtered,
        };

        return Some(after_version_max_filtered);
    }
    None
}

async fn do_list_languages(session: &Session, _matches: &ArgMatches) {
    if let Ok(mut all_languages) = languages(session).await {
        all_languages.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        for lang in all_languages {
            println!("- {} (id: {})", lang.name, lang.id);
        }
    }
}

async fn do_list_compilers(session: &Session, matches: &ArgMatches) {
    let name = matches.get_one::<String>("name");
    let lang = matches.get_one::<String>("language");
    let isa = matches.get_one::<String>("isa");
    let version_min = matches.get_one::<String>("version-min");
    let version_max = matches.get_one::<String>("version-max");

    let maybe_compilers = find_compilers(
        session,
        false,
        name.cloned(),
        lang.cloned(),
        isa.cloned(),
        version_min.cloned(),
        version_max.cloned(),
    )
    .await;
    if let Some(compilers) = maybe_compilers {
        for c in compilers {
            println!("- {}", c.to_text());
        }
    } else {
        println!("No compiler found");
    }
}

async fn do_compile(session: &Session, matches: &ArgMatches) {
    let is_summary = matches.get_one::<bool>("summary").unwrap();

    let mut filters_config = Filters::new()
        .binary(*matches.get_one("binary").unwrap())
        .binary_object(*matches.get_one("binary-object").unwrap())
        .execute(*matches.get_one("execute").unwrap());

    let mut stdout_f = match matches.get_one::<String>("stdout") {
        Some(s) if s == "-" => Some(Box::new(std::io::stdout()) as Box<dyn std::io::Write>),
        Some(filename) => std::fs::File::create(filename)
            .map(|f| Some(Box::new(f) as Box<dyn std::io::Write>))
            .unwrap(),
        _ => None,
    };

    let mut stderr_f = match matches.get_one::<String>("stderr") {
        Some(s) if s == "-" => Some(Box::new(std::io::stderr()) as Box<dyn std::io::Write>),
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

    let compilers_id = if let Some(id) = matches.get_one::<String>("compiler-id") {
        vec![get_compiler_info(session, id).await.unwrap()]
    } else {
        let name = matches.get_one::<String>("compiler-name");
        let lang = matches.get_one::<String>("compiler-lang");
        let isa = matches.get_one::<String>("compiler-isa");
        let version_min = matches.get_one::<String>("version-min");
        let version_max = matches.get_one::<String>("version-max");

        find_compilers(
            session,
            true,
            name.cloned(),
            lang.cloned(),
            isa.cloned(),
            version_min.cloned(),
            version_max.cloned(),
        )
        .await
        .unwrap()
        .into_iter()
        .collect::<Vec<CompilerInfo>>()
    };

    for compiler_info in compilers_id {
        // println!("{:?}", compiler_info);
        let compiler_id = compiler_info.id;
        let mut local_filters = filters_config.clone();
        if !compiler_info.supportsExecute.unwrap() {
            local_filters = local_filters.execute(false)
        }
        let simple_job = CompileJob::build(&source_data, &flags, &local_filters);

        let compile_ret1 = compile(session, &compiler_id, simple_job.clone()).await;

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
                "{} Compilation \"{}\" ({})",
                (if ret1.code == 0 {
                    "✔".green()
                } else {
                    "✗".red()
                }),
                compiler_info.name,
                ret1.code,
            );
        }
        if filters_config.execute != local_filters.execute {
            if !is_summary {
                println!("Execution not supported\n");
            } else {
                println!(
                    "{} Execution not supported for \"{}\".",
                    "✗".red(),
                    compiler_info.name,
                );
            }
        }
        if let Some(exec_result) = ret1.execResult {
            if !is_summary {
                println!("Execution:\n{}", exec_result.stdout.to_text());
            } else {
                println!(
                    "{} Execution \"{}\" ({})",
                    (if exec_result.code == 0 {
                        "✔".green()
                    } else {
                        "✗".red()
                    }),
                    compiler_info.name,
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
        .subcommand(Command::new("list-languages"))
        .subcommand(
            Command::new("list-compilers")
                .arg(Arg::new("all").action(clap::ArgAction::SetTrue).long("all"))
                .arg(Arg::new("name").long("name"))
                .arg(Arg::new("isa").long("instruction-set"))
                .arg(Arg::new("language").long("language"))
                .arg(Arg::new("version-min").long("version-min"))
                .arg(Arg::new("version-max").long("version-max")),
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
                .arg(
                    Arg::new("version-min")
                        .long("version-min")
                        .conflicts_with("compiler-id"),
                )
                .arg(
                    Arg::new("version-max")
                        .long("version-max")
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

    let base_url = matches
        .get_one::<String>("base-url")
        .expect("can't be missing");

    let session = Session {
        base_url: base_url.clone(),
        client: reqwest::Client::builder().user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        )).build()?,
        // all_compilers: None,
        // all_compilers_full: false,
    };

    match matches.subcommand() {
        Some(("compile", sub_matches)) => do_compile(&session, sub_matches).await,
        Some(("list-compilers", sub_matches)) => do_list_compilers(&session, sub_matches).await,
        Some(("list-languages", submatches)) => do_list_languages(&session, submatches).await,
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
