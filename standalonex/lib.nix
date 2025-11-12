{ pkgs
, self
,
}:

let
  parseStage0 = content:
    let
      lines = builtins.splitString "\n" content;
      parsedLines = builtins.filter (line: !(builtins.match "^\\s*#.*" line != null || builtins.match "^\\s*$" line != null)) lines;
      keyValues = builtins.map (line: builtins.splitString "=" line) parsedLines;
      toAttrSet = builtins.foldl' (acc: kv: acc // { "${builtins.head kv}" = builtins.elemAt kv 1; }) { } keyValues;
    in
    toAttrSet;

  stage0Attrs = parseStage0 (builtins.readFile (self + "/src/stage0"));
in
{
  parseStage0 = parseStage0;
  stage0Attrs = stage0Attrs;
}
