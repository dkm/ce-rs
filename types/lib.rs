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
#[derive(Deserialize, Debug, Clone)]
struct CompilerInfo {
    id: String,
    name: String,
    lang: String,
    compilerType: String,
    semver: String,
    instructionSet: String,

    supportsExecute: Option<bool>,

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
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone)]
struct CompileJob {
    source: String,
    options: CompileOptions,
    lang: Option<String>,
    allowStoreCodeDebug: bool,
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
