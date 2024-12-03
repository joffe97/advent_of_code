import Data.Function
import Data.List (findIndices, isPrefixOf)
import Data.List.Split
import Text.Regex.Posix

main :: IO ()
main = do
  content <- readFile "input"
  print
    ( content
        & getAllDoSegments
        & concatMap getMulStrs
        & map calculateMulStr
        & sum
    )

getNumbersInMulStr :: String -> [Int]
getNumbersInMulStr x = map read (getAllTextMatches (x =~ "[0-9]{1,3}") :: [String])

calculateMulStr :: String -> Int
calculateMulStr = product . getNumbersInMulStr

stripBeforeDo :: String -> String
stripBeforeDo x = if "do()" `isPrefixOf` x then x else splitOn "do()" x & tail & concat

getAllDoSegments :: String -> [String]
getAllDoSegments x = splitOn "don't()" ("do()" ++ x) & map stripBeforeDo

getMulStrs :: String -> [String]
getMulStrs x = getAllTextMatches (x =~ "mul\\([0-9]{1,3}\\,[0-9]{1,3}\\)")
