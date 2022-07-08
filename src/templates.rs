#![allow(dead_code)]

pub static CLANGD: &str = r#"
CompileFlags:                   
  Add: [-xc++, -Wall, -std=c++11, -fdiagnostics-color=always, -Wpedantic, -Werror, -Wshadow, -Wformat=2, -Wconversion, -Wunused-parameter]

Diagnostics:
  UnusedIncludes: None #Possible values: None, Strict
  ClangTidy: # Checklist can be found here: https://clang.llvm.org/extra/clang-tidy/checks/list.html
    Add: modernize*
    Remove: [hicpp-braces-around-statements, modernize-use-trailing-return-type]

Hover:
   ShowAKA: Yes

InlayHints:
  Enabled: No
  ParameterNames: No
  DeducedTypes: No
"#;

pub static CLANG_FORMAT: &str = r#"
BasedOnStyle: LLVM
IndentWidth: 4
TabWidth: 4
UseTab: Always

AllowShortIfStatementsOnASingleLine: true
NamespaceIndentation: All
Language: Cpp
DerivePointerAlignment: false
PointerAlignment: Left

AccessModifierOffset: -4
AlignAfterOpenBracket: true
AlignConsecutiveAssignments: true
AlignConsecutiveDeclarations: false
AlignEscapedNewlinesLeft: false
AlignOperands:   true
AlignTrailingComments: false
AllowAllParametersOfDeclarationOnNextLine: true
AllowShortBlocksOnASingleLine: false
AllowShortCaseLabelsOnASingleLine: false
AllowShortFunctionsOnASingleLine: Empty
AllowShortIfStatementsOnASingleLine: false
AllowShortLoopsOnASingleLine: false
AlwaysBreakAfterDefinitionReturnType: false
AlwaysBreakAfterReturnType: None
AlwaysBreakBeforeMultilineStrings: false
AlwaysBreakTemplateDeclarations: true
BinPackArguments: false
BinPackParameters: false

BraceWrapping: {
  AfterClass: 'true'
  AfterControlStatement: 'true'
  AfterEnum : 'true'
  AfterFunction : 'true'
  AfterNamespace : 'true'
  AfterStruct : 'true'
  AfterUnion : 'true'
  BeforeCatch : 'true'
  BeforeElse : 'true'
  IndentBraces : 'false'
  AfterExternBlock : 'true'
  SplitEmptyFunction : 'false'
  SplitEmptyRecord : 'false'
  SplitEmptyNamespace : 'true'
}
"#;

pub static CLANG_TIDY: &str = r#"
Checks:              '-checks=-*,clang-analyzer-*,-clang-analyzer-cplusplus*'
WarningsAsErrors:    true
HeaderFilterRegex:   '.*'
FormatStyle:         LLVM
InheritParentConfig: true
User:                user
CheckOptions:
  - { key: readability-identifier-naming.VariableCase,        value: lower_case }
  - { key: readability-identifier-naming.GlobalConstantCase,  value: UPPER_CASE }
  - { key: readability-identifier-naming.FunctionCase,        value: lower_case }
"#;