Command line client for Compiler-Explorer
=========================================

`ce-rs` is a simple command line client for [`Compiler-Explorer`]. It can't do
everything the web UI can, but it can help with simple tasks.

## Building

`ce-rs` is written in Rust.

``` sh
$ cargo build
```

## How to use it

### Listing the compilers

``` sh
Usage: ce-rs list-compilers [OPTIONS]

Options:
      --all                        
      --name <name>                
      --instruction-set <isa>      
      --language <language>        
      --version-min <version-min>  
      --version-max <version-max>  
  -h, --help                       Print help
```

You can query the installed compilers for some language and/or other critera:

``` sh
$ ce-rs list-compilers  --language rust  --version-min 1.60 --version-max 1.64
- "rustc 1.60.0", id: r1600, language: rust, type: rust, version: 1.60.0, ISA: amd64
- "rustc 1.61.0", id: r1610, language: rust, type: rust, version: 1.61.0, ISA: amd64
- "rustc 1.62.0", id: r1620, language: rust, type: rust, version: 1.62.0, ISA: amd64
- "rustc 1.63.0", id: r1630, language: rust, type: rust, version: 1.63.0, ISA: amd64
- "rustc 1.64.0", id: r1640, language: rust, type: rust, version: 1.64.0, ISA: amd64
```

### Compiling code

``` sh
Usage: ce-rs compile [OPTIONS] <--source <source>|--source-file <source-file>>

Options:
      --source <source>                 
      --binary                          
      --binary-object                   
      --execute                         
      --summary                         
      --source-file <source-file>       
      --id <compiler-id>                
      --name <compiler-name>            
      --language <compiler-lang>        
      --instruction-set <compiler-isa>  
      --version-min <version-min>       
      --version-max <version-max>       
      --flags <flags>                   
      --stdout <stdout>                 Write stdout to given file (stdout if -)
      --stderr <stderr>                 Write stderr to given file (stdout if -)
  -f, --filters <filters>               
  -h, --help                            Print help
```

Same filtering as for the `list-compilers` applies and can be used to compile a
single source with several compilers. The `--summary` gives a synthetic output:

``` sh
$ ce-rs compile --source-file toto.rs  --language rust  --version-min 1.60 --version-max 1.64 --summary --stderr - --stdout -
✔ Compilation "rustc 1.60.0" (0)
✔ Compilation "rustc 1.61.0" (0)
✔ Compilation "rustc 1.62.0" (0)
✔ Compilation "rustc 1.63.0" (0)
✔ Compilation "rustc 1.64.0" (0)
```

With `--execute`:
``` sh
$ ce-rs compile --source-file toto.rs  --language rust \
   --version-min 1.60 --version-max 1.64 \
   --summary --stderr - --stdout - \
   --execute
✔ Compilation "rustc 1.60.0" (0)
✔ Execution "rustc 1.60.0" (0)
✔ Compilation "rustc 1.61.0" (0)
✔ Execution "rustc 1.61.0" (0)
✔ Compilation "rustc 1.62.0" (0)
✔ Execution "rustc 1.62.0" (0)
✔ Compilation "rustc 1.63.0" (0)
✔ Execution "rustc 1.63.0" (0)
✔ Compilation "rustc 1.64.0" (0)
✔ Execution "rustc 1.64.0" (0)
```

For example, when investigating a regression, you can use a similar command to check on older versions:

``` sh
$ ce-rs compile --source-file pr56843.C \
   --flags "-O2 -fno-unit-at-a-time -fwhole-program" \
   --summary  --language 'c\+\+' \
   --name 'gcc' --instruction-set amd64 \
   --version-min 9.0 --version-max 13.2
✔ Compilation "MinGW gcc 11.3.0" (0)
✔ Compilation "MinGW gcc 12.1.0" (0)
✔ Compilation "MinGW gcc 12.2.0" (0)
✔ Compilation "MinGW gcc 13.1.0" (0)
✔ Compilation "x86-64 gcc 10.1" (0)
✔ Compilation "x86-64 gcc 10.2" (0)
✔ Compilation "x86-64 gcc 10.3" (0)
✔ Compilation "x86-64 gcc 10.4" (0)
✔ Compilation "x86-64 gcc 10.5" (0)
✔ Compilation "x86-64 gcc 11.1" (0)
✔ Compilation "x86-64 gcc 11.2" (0)
✔ Compilation "x86-64 gcc 11.3" (0)
✔ Compilation "x86-64 gcc 11.4" (0)
✔ Compilation "x86-64 gcc 12.1" (0)
✔ Compilation "x86-64 gcc 12.2" (0)
✔ Compilation "x86-64 gcc 12.3" (0)
✔ Compilation "x86-64 gcc 13.1" (0)
✔ Compilation "x86-64 gcc 13.2" (0)
✔ Compilation "x86-64 gcc 9.1" (0)
✔ Compilation "x86-64 gcc 9.2" (0)
✔ Compilation "x86-64 gcc 9.3" (0)
✔ Compilation "x86-64 gcc 9.4" (0)
✔ Compilation "x86-64 gcc 9.5" (0)
```


[`Compiler-Explorer`]: https://compiler-explorer.org
