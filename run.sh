wget -q -nc https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar
java -jar antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust src/grammar/Regex.g4 -visitor
