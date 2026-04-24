import type { IconName } from '@/components/ui/Icon';

// ---------------------------------------------------------------------------
// Language / extension → NerdFont icon name mapping
// ---------------------------------------------------------------------------
const EXTENSION_TO_ICON: Record<string, IconName> = {
  // TypeScript / JavaScript
  ts: 'file-typescript',
  tsx: 'file-reactts',
  js: 'file-javascript',
  jsx: 'file-reactjs',
  mjs: 'file-javascript',
  cjs: 'file-javascript',

  // JSON family
  json: 'file-json',
  jsonc: 'file-json',
  json5: 'file-json',

  // Web markup
  html: 'file-html',
  htm: 'file-html',

  // Styles
  css: 'file-css',
  scss: 'file-sass',
  sass: 'file-sass',
  less: 'file-less',
  styl: 'file-stylus',

  // Front‑end frameworks / meta
  vue: 'file-vue',
  svelte: 'file-svelte',
  astro: 'file-astro',

  // Rust
  rs: 'file-rust',

  // Python
  py: 'file-python',
  pyw: 'file-python',

  // Go
  go: 'file-go',

  // Ruby
  rb: 'file-ruby',

  // Java (JVM)
  java: 'file-java',
  class: 'file-java',
  jar: 'file-java',

  // C# / .NET
  cs: 'file-csharp',
  csproj: 'file-csharp',

  // F#
  fs: 'file-fsharp',
  fsx: 'file-fsharp',

  // PHP
  php: 'file-php',
  phtml: 'file-php',
  twig: 'file-twig',

  // Swift
  swift: 'file-swift',

  // Kotlin
  kt: 'file-kotlin',
  kts: 'file-kotlin',

  // Scala
  scala: 'file-scala',

  // Dart
  dart: 'file-dart',

  // Zig
  zig: 'file-zig',

  // Nim
  nim: 'file-nim',

  // Crystal
  cr: 'file-crystal',

  // Elixir / Erlang
  ex: 'file-elixir',
  exs: 'file-elixir',
  erl: 'file-erlang',
  hrl: 'file-erlang',

  // Clojure
  clj: 'file-clojure',
  cljs: 'file-clojure',
  edn: 'file-clojure',

  // Haskell
  hs: 'file-haskell',
  lhs: 'file-haskell',

  // Lua
  lua: 'file-lua',

  // Perl / Raku
  pl: 'file-perl',
  pm: 'file-perl',
  t: 'file-perl',
  raku: 'file-perl',
  rakumod: 'file-perl',

  // R / Data
  r: 'file-r',
  R: 'file-r',
  sql: 'file-sql',

  // Shell
  sh: 'file-shell',
  bash: 'file-shell',
  zsh: 'file-shell',
  fish: 'file-shell',
  ps1: 'file-powershell',
  psm1: 'file-powershell',
  psd1: 'file-powershell',
  bat: 'file-batch',
  cmd: 'file-batch',

  // Configuration & markup
  toml: 'file-toml',
  yaml: 'file-yaml',
  yml: 'file-yaml',
  ini: 'file-settings',
  cfg: 'file-settings',
  conf: 'file-settings',
  env: 'file-env',

  // Infrastructure
  tf: 'file-terraform',
  tfvars: 'file-terraform',
  hcl: 'file-terraform',
  dockerfile: 'file-docker',
  Dockerfile: 'file-docker',
  dockerignore: 'file-docker',

  // Build
  makefile: 'file-makefile',
  Makefile: 'file-makefile',
  cmake: 'file-makefile',

  // Lock / VCS
  lock: 'file-lock',
  gitignore: 'file-git',
  gitattributes: 'file-git',
  gitmodules: 'file-git',

  // Lint / format
  eslintrc: 'file-eslint',
  prettierrc: 'file-prettier',
  babelrc: 'file-babel',
  postcssrc: 'file-postcss',
  editorconfig: 'file-editorconfig',

  // GraphQL
  graphql: 'file-graphql',
  gql: 'file-graphql',

  // Protobuf
  proto: 'file-protobuf',

  // Documentation / text
  md: 'file-markdown',
  mdx: 'file-markdown',
  txt: 'file-text',
  csv: 'file-csv',
  xml: 'file-xml',
  svg: 'file-svg',

  // C / C++
  c: 'file-c',
  cpp: 'file-cpp',
  cxx: 'file-cpp',
  cc: 'file-cpp',
  h: 'file-c',
  hpp: 'file-cpp',
  hxx: 'file-cpp',
  hh: 'file-cpp',

  // Objective‑C
  m: 'file-objectivec',
  mm: 'file-objectivec',

  // Assembly
  asm: 'file-assembly',
  s: 'file-assembly',
  nasm: 'file-assembly',

  // Functional / esoteric
  ml: 'file-ocaml',
  mli: 'file-ocaml',
  elm: 'file-elm',
  purs: 'file-purescript',

  // LaTeX
  tex: 'file-tex',
  bib: 'file-bibtex',
  rst: 'file-rst',
  adoc: 'file-asciidoc',

  // Platform‑specific
  vagrant: 'file-vagrant',
  pod: 'file-kubernetes',
  service: 'file-kubernetes',
};

// Known filenames that should have a specific icon regardless of extension
const FILENAME_TO_ICON: Record<string, IconName> = {
  'package.json': 'file-json',
  'package-lock.json': 'file-lock',
  'yarn.lock': 'file-lock',
  'pnpm-lock.yaml': 'file-lock',
  'Dockerfile': 'file-docker',
  'Makefile': 'file-makefile',
  '.gitignore': 'file-git',
  '.dockerignore': 'file-docker',
  '.editorconfig': 'file-editorconfig',
  '.prettierrc': 'file-prettier',
  '.eslintrc': 'file-eslint',
  '.babelrc': 'file-babel',
  '.env': 'file-env',
  '.env.example': 'file-env',
  'tsconfig.json': 'file-typescript',
  'jsconfig.json': 'file-javascript',
  'composer.json': 'file-php',
  'Podfile': 'file-ruby',
  'Gemfile': 'file-ruby',
  'Rakefile': 'file-ruby',
  'Vagrantfile': 'file-vagrant',
};

export function getLanguageIcon(path: string): IconName {
  const name = path.split(/[/\\]/).pop() || '';
  const exact = FILENAME_TO_ICON[name];
  if (exact) return exact;

  const ext = name.includes('.') ? name.split('.').pop()!.toLowerCase() : '';
  // Handle dotfiles like .eslintrc whose "extension" is the whole filename
  if (name.startsWith('.')) {
    const dotExt = name.substring(1).toLowerCase();
    const dotIcon = EXTENSION_TO_ICON[dotExt];
    if (dotIcon) return dotIcon;
  }

  const result = EXTENSION_TO_ICON[ext];
  return result ?? 'file';
}
