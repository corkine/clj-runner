# CLJ Runner

A simple way to 'click to run' clojure script with deps.edn local deps cache.

> Just Windows work now, need JVM install and Clojure CLI Configured.

Generally speaking, in order to accomplish a certain task, you need to create a Clojure project in IDEA or VSCode - maybe Lein or deps.edn manage the dependencies. When development is done, you need to package uberjar and create a script: `.bat` for Windows or `.sh` script for Linux to execute `java -jar clojure.main YOUR-SCRIPT` or `clojure -M -m YOUR-SCRIPT`, which is pretty tedious -- if you want to configure something, you will have to use one more edn file, which you then read in the Clojure script - now you have four things: `YOUR-SCRIPT.clj`, `YOUR-CONFIG.edn`, `YOUR-DEPS-ALL.jar` and `YOUR-RUNNABLE.bat`, which is unnecessary for most tasks.

Since Clojure scripts are generally used to complete the daily low-frequency developer work, assuming that the developer has installed the JVM and clojure CLI (basically necessary), then only one `YOUR-ONLY-SCRIPT.clj` script + `clj-runner` is needed. configure Windows to double-click to open the `.clj` file to run `clj-runner.exe`, you don’t need to modify the registry to enjoy the convenience of double-click execution, scheduled task execution, and automatic execution of Clojure scripts at startup. 

clj-runner.exe uses Go development, what it does is very simple, **read the command from the first line of the Clojure script, and then execute it**, so *just add a command similar to the following to the head of the Clojure script to execute - need to expose the current CWD for lookup directory, add th*e dependencies required by the script, and run the required functions:

```powershell
;clojure -Sdeps '{:paths ["".""] :deps {clj-file-zip/clj-file-zip {:mvn/version,""0.1.0""}}}' -M -m auto-backup```