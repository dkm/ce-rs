use crate::types::*;

impl CompilerInfo {
    pub fn to_text(&self) -> String {
        format!(
            "\"{}\", id: {}, language: {}, type: {}, version: {}, ISA: {}",
            self.name, self.id, self.lang, self.compilerType, self.semver, self.instructionSet
        )
    }
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

impl SomeOutput {
    pub fn to_text(&self) -> String {
        self.0
            .iter()
            .map(|x| x.text.clone())
            .collect::<Vec<String>>()
            .join("\n")
    }
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
