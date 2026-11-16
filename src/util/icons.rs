use std::path::Path;

// Nerd Font glyphs (nf-dev, nf-seti, nf-fa, nf-oct sets). Written as escapes
// because editors and diffs tend to mangle private-use characters.
const FOLDER: &str = "\u{f07b}";
const FILE: &str = "\u{f15b}";
const BINARY: &str = "\u{eae8}";
const GIT: &str = "\u{e702}";
const CONFIG: &str = "\u{e615}";
const IMAGE: &str = "\u{f1c5}";
const AUDIO: &str = "\u{f1c7}";
const VIDEO: &str = "\u{f1c8}";
const ARCHIVE: &str = "\u{f1c6}";
const DOC: &str = "\u{f0f6}";
const LOCK: &str = "\u{f023}";
const SHELL: &str = "\u{e795}";

/// Pick a Nerd Font icon for an entry. Directories get a folder, then exact
/// filenames are checked (these often have no useful extension), then the
/// lowercased extension, and anything unknown falls back to a generic file.
pub fn icon_for(name: &str, is_dir: bool) -> &'static str {
    if is_dir {
        return match name {
            ".git" => GIT,
            _ => FOLDER,
        };
    }

    let lower = name.to_ascii_lowercase();
    match lower.as_str() {
        "cargo.toml" | "cargo.lock" => return "\u{e7a8}",
        "makefile" | "gnumakefile" | "cmakelists.txt" => return "\u{e779}",
        "dockerfile" | "docker-compose.yml" | "docker-compose.yaml" | ".dockerignore" => {
            return "\u{f308}";
        }
        ".gitignore" | ".gitattributes" | ".gitmodules" | ".gitconfig" => return GIT,
        "license" | "licence" | "copying" => return "\u{f15c}",
        "readme" | "readme.md" => return "\u{f48a}",
        ".bashrc" | ".bash_profile" | ".zshrc" | ".zprofile" | ".profile" | ".zshenv" => {
            return SHELL;
        }
        ".env" | ".envrc" => return "\u{f462}",
        "package.json" | "package-lock.json" => return "\u{e71e}",
        _ => {}
    }

    let ext = Path::new(&lower)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match ext {
        // systems languages
        "rs" => "\u{e7a8}",
        "c" | "h" => "\u{e61e}",
        "cpp" | "cc" | "cxx" | "c++" | "hpp" | "hh" | "hxx" | "ino" => "\u{e61d}",
        "go" => "\u{e627}",
        "zig" => "\u{e6a9}",
        "asm" | "s" => "\u{e637}",
        "cs" | "csx" => "\u{e648}",
        "swift" => "\u{e755}",
        "d" => "\u{e7af}",
        // jvm
        "java" | "jar" | "class" => "\u{e738}",
        "kt" | "kts" => "\u{e634}",
        "scala" | "sc" => "\u{e737}",
        "groovy" | "gradle" => "\u{e775}",
        "clj" | "cljs" | "cljc" | "edn" => "\u{e768}",
        // scripting
        "py" | "pyw" | "pyi" | "pyc" | "ipynb" => "\u{e73c}",
        "rb" | "erb" | "gemspec" => "\u{e739}",
        "pl" | "pm" | "t" => "\u{e769}",
        "lua" => "\u{e620}",
        "php" => "\u{e73d}",
        "r" | "rmd" => "\u{e68a}",
        "jl" => "\u{e624}",
        "sh" | "bash" | "zsh" | "fish" | "ksh" => SHELL,
        "ps1" | "psm1" | "bat" | "cmd" => "\u{ebc4}",
        "vim" => "\u{e62b}",
        // functional
        "hs" | "lhs" => "\u{e61f}",
        "ml" | "mli" => "\u{e67a}",
        "ex" | "exs" | "heex" => "\u{e62d}",
        "erl" | "hrl" => "\u{e7b1}",
        "elm" => "\u{e62c}",
        "lisp" | "el" | "scm" | "rkt" => "\u{e6b0}",
        "nix" => "\u{f313}",
        "dart" => "\u{e798}",
        // web
        "js" | "mjs" | "cjs" => "\u{e74e}",
        "ts" | "mts" | "cts" => "\u{e628}",
        "tsx" | "jsx" => "\u{e7ba}",
        "html" | "htm" | "xhtml" => "\u{e736}",
        "css" => "\u{e749}",
        "scss" | "sass" => "\u{e603}",
        "less" => "\u{e758}",
        "vue" => "\u{e6a0}",
        "svelte" => "\u{e697}",
        "wasm" | "wat" => "\u{e6a1}",
        // data / config
        "json" | "jsonc" | "json5" => "\u{e60b}",
        "toml" | "yaml" | "yml" | "ini" | "cfg" | "conf" | "properties" => CONFIG,
        "xml" | "xsl" | "plist" => "\u{e619}",
        "csv" | "tsv" => "\u{f1c3}",
        "sql" | "db" | "sqlite" | "sqlite3" => "\u{f1c0}",
        "env" => "\u{f462}",
        "lock" => LOCK,
        // docs
        "md" | "markdown" | "mdx" => "\u{e73e}",
        "txt" | "log" | "rtf" => "\u{f15c}",
        "pdf" => "\u{f1c1}",
        "doc" | "docx" | "odt" => "\u{f1c2}",
        "xls" | "xlsx" | "ods" => "\u{f1c3}",
        "ppt" | "pptx" | "odp" => "\u{f1c4}",
        "tex" | "bib" => "\u{e69b}",
        "rst" | "org" | "adoc" => DOC,
        // media
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "ico" | "tiff" | "heic" | "avif"
        | "svg" => IMAGE,
        "mp3" | "wav" | "flac" | "ogg" | "m4a" | "aac" | "opus" => AUDIO,
        "mp4" | "mkv" | "mov" | "avi" | "webm" | "flv" => VIDEO,
        "ttf" | "otf" | "woff" | "woff2" => "\u{f031}",
        // archives
        "zip" | "tar" | "gz" | "tgz" | "bz2" | "xz" | "zst" | "7z" | "rar" | "dmg" | "iso" => {
            ARCHIVE
        }
        // binaries
        "exe" | "bin" | "o" | "a" | "so" | "dylib" | "dll" | "out" | "msi" | "app" => BINARY,
        // misc
        "diff" | "patch" => GIT,
        "key" | "pem" | "crt" | "cer" | "pub" => "\u{f084}",
        _ => FILE,
    }
}
