# CLJ Runner

A simple way to 'click to run' clojure script with deps.edn local deps cache on Windows.

> Need `babashka 1.0` (and `JDK` if you want to use non AOT Java class libraries), no `clojure CLI for Windows` need.

## The Idea of CLJ Runner

Generally speaking, in order to accomplish a certain task, you need to create a Clojure project in IDE like IDEA or VSCode - use lein project.clj or deps.edn manage the dependencies. When development is done, you need to package uberjar and create a script: `.bat` for Windows or `.sh` script for Linux to execute `java -jar .. clojure.main YOUR-SCRIPT` or `clojure -cp .. -M -m YOUR-SCRIPT`, which is good for terminal user (if JVM is necessary), but not good for most script use, which in most case point dependencies is enough, so use clojure cli to manage dependencies and run script is the best idea, isn't it?

No. There are two reasons. First, **clojure cli is not available across platforms**. In the unix like system, it is written by bash, while in Windows, it can only run in a higher version of the powershell. Second, even if we get a cross platform clojure cli, such as deps.clj, which is built into babashka and exposed through the 'bb clojure' subcommand, **it is tedious to point out the dependent library before executing the script**. Fortunately, there is a solution: load the command through the env program: add `#/usr/bin/env bb clojure -Sdeps xxx ...` That is, env will automatically spell the file name after it to execute the complete command.

In order to make Clojure and the JVM library behind it provide a 100 times better experience than shell scripts: ① **The installation should be as simple as possible**. After babashka 1.0, the built-in deps.clj can completely replace clojure cli, so you only need to install JDK and babashka (see below). ② **The operation should be as fast as possible, and there should be as many compatible class libraries as possible**. For the "pure" Clojure script of the babashka built-in library, it can be run in the GraalVM AOT optimized SVM through the `bb` command. JDK is not required, and the startup speed and memory consumption are minimal, but the capacity is limited. For programs that rely on Java class libraries that cannot be AOT, the introduction of Maven dependency through the `bb clojure` command can provide more long-term, efficient, and compatible running code in the JVM. **This two-layer model gives consideration to speed, development and running efficiency and compatibility, making Clojure a highly available JVM script**.

And all this just needs `#/usr/bin/env bb` or` #/usr/bin/env bb clojure -Sdeps` -- in UNIX like system. For Windows, to support similar "Click to run", the clj runner program will automatically recognize the first line beginning with `#!/usr/bin/env` and then splicing file name to Powershell to run the command.

## Impl - Go

Go version of clj-runner on clj_runner.go, managed by go.mod.

```go
go build
```

## Impl - Rust

Rust version of clj-runner on src/main.rs, managed by Cargo.toml and Cargo.lock.

```rust
cargo run -- xxx.clj
cargo build --release
```

## Usage

clj-runner support: 

```clj
// Universe Script on Windows, Unix-like OS:
#!/usr/bin/env bb
#!/usr/bin/env bb clojure -Sdeps '{:paths ["."] :deps {clj-file-zip/clj-file-zip {:mvn/version,"0.1.0"}}}' -M -m auto-backup

// Simple Style:
; bb
; bb clojure -Sdeps '{:paths ["."] :deps {clj-file-zip/clj-file-zip {:mvn/version,"0.1.0"}}}' -M -m auto-backup
; clojure -Sdeps '{:paths ["."] :deps {clj-file-zip/clj-file-zip {:mvn/version,"0.1.0"}}}' -M -m auto-backup
```

Note. when use babashka, you may need run -main like python's `if __name__ == '__main__':` with that:

```clojure
(defn -main []
  (println "running script files-metadata-upload...")
  (let [files (collect-files)]
    (println (send-request! files))))

(when (= "SCI" (-> *clojure-version* :qualifier))
  (-main))
```