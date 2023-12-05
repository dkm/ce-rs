pub mod implems;

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

// #[allow(dead_code)]
// #[allow(non_snake_case)]
// #[derive(Deserialize, Debug)]
// pub struct Language {
//     pub id: String,
//     pub name: String,
//     pub extensions: Vec<String>,
//     pub monaco: String,

// }

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Language {
    pub id: String,
    pub name: String,
    pub monaco: String,
    // FIXME
    pub extensions: Vec<String>,
    pub alias: Option<Vec<String>>,
    pub formatter: Option<String>,
    pub supportsExecute: Option<bool>,
    pub logoUrl: Option<String>,
    pub logoUrlDark: Option<String>,
    //pub logoData: Option<FIXME>,
    //pub logoDataDark: Option<FIXME>,
    pub example: Option<String>,
    // Should be Regex?
    pub previewFilter: Option<String>,
    pub monacoDisassembly: Option<String>,
    pub tooltip: Option<String>,
    pub defaultCompiler: Option<String>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct CompilerInfo {
    pub id: String,
    pub name: String,
    pub lang: String,
    pub compilerType: String,
    pub semver: String,
    pub instructionSet: String,

    pub exe: Option<String>,
    pub version: Option<String>,
    pub fullVersion: Option<String>,
    pub baseName: Option<String>,
    pub alias: Option<Vec<String>>,
    pub options: Option<String>,
    //pub versionFlag: Option<Vec<String>>,
    pub versionRe: Option<String>,
    pub explicitVersion: Option<String>,

    pub compilerCategories: Option<Vec<String>>,
    pub debugPatched: Option<bool>,
    pub demangler: Option<String>,
    pub demanglerType: Option<String>,
    pub demanglerArgs: Option<Vec<String>>,
    pub objdumper: Option<String>,
    pub objdumperType: Option<String>,
    pub objdumperArgs: Option<Vec<String>>,
    pub intelAsm: Option<String>,
    pub supportsAsmDocs: Option<bool>,
    pub needsMulti: Option<bool>,
    pub adarts: Option<String>,
    pub supportsDeviceAsmView: Option<bool>,
    pub supportsDemangle: Option<bool>,
    pub supportsBinary: Option<bool>,
    pub supportsBinaryObject: Option<bool>,
    pub supportsIntel: Option<bool>,
    pub interpreted: Option<bool>,
    pub supportsExecute: Option<bool>,
    pub supportsGccDump: Option<bool>,
    pub supportsFiltersInBinary: Option<bool>,
    pub supportsOptOutput: Option<bool>,
    pub supportsStackUsageOutput: Option<bool>,
    pub supportsPpView: Option<bool>,
    pub supportsAstView: Option<bool>,
    pub supportsIrView: Option<bool>,
    pub supportsLLVMOptPipelineView: Option<bool>,

    pub supportsRustMirView: Option<bool>,
    pub supportsRustMacroExpView: Option<bool>,
    pub supportsRustHirView: Option<bool>,
    pub supportsHaskellCoreView: Option<bool>,
    pub supportsHaskellStgView: Option<bool>,
    pub supportsHaskellCmmView: Option<bool>,
    pub supportsCfg: Option<bool>,
    pub supportsGnatDebugViews: Option<bool>,
    pub supportsLibraryCodeFilter: Option<bool>,
    pub supportsMarch: Option<bool>,
    pub supportsTarget: Option<bool>,
    pub supportsTargetIs: Option<bool>,
    pub executionWrapper: Option<String>,
    pub executionWrapperArgs: Option<Vec<String>>,

    pub postProcess: Option<Vec<String>>,
    pub group: Option<String>,
    pub groupName: Option<String>,
    pub includeFlag: Option<String>,
    pub includePath: Option<String>,
    pub linkFlag: Option<String>,
    pub rpathFlag: Option<String>,
    pub libpathFlag: Option<String>,
    pub libPath: Option<Vec<String>>,
    pub ldPath: Option<Vec<String>>,
    pub extraPath: Option<Vec<String>>,
    // envVars: [string, string][],
    pub notification: Option<String>,
    pub isSemVer: Option<bool>,
    pub isNightly: Option<bool>,
    // libsArr: Library['id'][],
    // tools: Record<ToolInfo['id'], Tool>,
    pub unwiseOptions: Option<Vec<String>>,
    pub hidden: Option<bool>,
    //buildenvsetup: Anon0,
    //license: Anon1,
    //remote: Anon2,
    // possibleOverrides: Option<AllCompilerOverrideOptions>,
    // possibleRuntimeTools: Option<PossibleRuntimeTools>,
    pub disabledFilters: Option<Vec<String>>,
    pub optArg: Option<String>,
    pub stackUsageArg: Option<String>,
    // externalparser: FIXME,
    pub removeEmptyGccDump: Option<bool>,
    pub irArg: Option<Vec<String>>,
    pub minIrArgs: Option<Vec<String>>,
    pub llvmOptArg: Option<Vec<String>>,
    pub llvmOptModuleScopeArg: Option<Vec<String>>,
    pub llvmOptNoDiscardValueNamesArg: Option<Vec<String>>,
    //cachedPossibleArguments: Option<FIXME>,
    pub nvdisasm: Option<String>,
    //mtime: Option<FIXME>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Session {
    pub id: u8,
    pub language: String,
    pub source: String,
    pub conformanceview: bool,
    pub compilers: Vec<CompilerConfig>,
    pub executors: Vec<ExecutorConfig>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ShortLinkInfo {
    pub sessions: Vec<Session>,
    pub trees: Vec<Tree>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Tree {}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ExecutorConfig {}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CompilerConfig {
    pub _internalid: u8,
    pub id: String,
    pub options: String,
    pub filters: Filters,
    pub libs: Vec<Library>,
    pub specialoutputs: Vec<Output>,
    pub tools: Vec<Tool>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Output {}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Library {}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tool {}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Filters {
    pub binary: bool,
    pub binaryObject: bool,
    pub commentOnly: bool,
    pub demangle: bool,
    pub directives: bool,
    pub execute: bool,
    pub intel: bool,
    pub labels: bool,
    pub libraryCode: bool,
    pub trim: bool,
    pub debugCalls: bool,
}
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone)]
pub struct CompileJob {
    pub source: String,
    pub options: CompileOptions,
    pub lang: Option<String>,
    pub allowStoreCodeDebug: bool,
}
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone)]
pub struct CompileOptions {
    pub userArguments: String,
    pub compilerOptions: OtherCompilerOptions,
    pub filters: Filters,
    pub tools: Vec<Tool>,
    pub libraries: Vec<Library>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OtherCompilerOptions {
    pub skipAsm: bool,
    pub executorRequest: bool,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Download {}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ToolResult {}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Label {}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Tag {
    pub line: i32,
    pub column: i32,
    pub text: String,
    pub severity: i32,
    pub file: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct SomeOutput(Vec<OutputItem>);

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct OutputItem {
    pub text: String,
    pub tag: Option<Tag>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct SourceLocation {
    pub file: Option<String>,
    pub line: i32,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AsmOutput(Vec<AsmOutputItem>);

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AsmOutputItem {
    pub text: String,
    pub source: Option<SourceLocation>,
    pub labels: Vec<Label>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct PopularArgument {
    pub description: String,
    pub timesused: i32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct CompileJobResult {
    pub inputFilename: String,
    pub code: i32,
    pub okToCache: bool,
    pub timedOut: bool,
    pub stdout: SomeOutput,
    pub stderr: SomeOutput,
    pub truncated: bool,
    pub execTime: String, // why not integer?
    pub processExecutionResultTime: f32,
    pub compilationOptions: Vec<String>,
    pub downloads: Vec<Download>,
    pub tools: Vec<ToolResult>,
    pub asm: AsmOutput,
    pub labelDefinitions: HashMap<String, i32>,
    pub parsingTime: String, // why not integer?
    pub filteredCount: i32,
    pub popularArguments: Option<HashMap<String, PopularArgument>>,
    pub execResult: Option<ExecutionResult>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ExecutionResult {
    pub code: i32,
    pub okToCache: Option<bool>,
    pub timedOut: bool,
    pub stdout: SomeOutput,
    pub stderr: SomeOutput,
    pub truncated: Option<bool>,
    pub execTime: Option<String>,
    pub processExecutionResultTime: Option<f32>,
    pub didExecute: bool,
    pub buildResult: ExecBuildResult,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ExecBuildResult {
    pub inputFilename: String,
    pub code: i32,
    pub okToCache: bool,
    pub timedOut: bool,
    pub stdout: SomeOutput,
    pub stderr: SomeOutput,
    pub truncated: bool,
    pub execTime: String,
    pub processExecutionResultTime: f32,
    pub downloads: Vec<Download>,
    pub executableFilename: String,
    pub compilationOptions: Vec<String>,
}
