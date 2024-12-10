app [main] {
    pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.17.0/lZFLstMUCUvd5bjnnpYromZJXkQUrdhbva4xdBInicE.tar.br",
}

import pf.Stdin
import pf.Stdout

main = 
    lines = input! |> Str.splitOn "\n"

    parts = List.map lines \line -> 
        Str.splitOn line " " 
            |> List.keepIf \s -> 
                !(Str.isEmpty s)

    a = List.map parts \p -> when List.first p |> Result.try Str.toI32 is
        Ok n -> n
        Err _ -> crash ""
    b = List.map parts \p -> when List.last p |> Result.try Str.toI32 is
        Ok n -> n
        Err _ -> crash ""

    sortedA = List.sortAsc a
    sortedB = List.sortAsc b

    absDiffs = List.map2 sortedA sortedB \a1, b1 -> Num.abs (a1 - b1) 
    sum = List.walk absDiffs 0 Num.add

    similarities = List.map sortedA \a1 -> a1 * (Num.toI32 (List.countIf sortedB \b1 -> b1 == a1))
    sum2 = List.walk similarities 0 Num.add

    Stdout.line! (Num.toStr sum)
    Stdout.line (Num.toStr sum2)

input : Task Str _
input = 
    bytes = Stdin.readToEnd! {}

    Str.fromUtf8 bytes
        |> Task.fromResult
