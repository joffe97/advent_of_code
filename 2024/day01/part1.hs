import Data.List
import Data.List.Split
import System.IO

main = do
  contents <- readFile "input"
  print
    . sum
    . map (abs . difference)
    . transpose
    . map sort
    . transpose
    . map readLine
    . lines
    $ contents

readLine :: String -> [Int]
readLine =
  map read
    . splitOn "   "

difference :: [Int] -> Int
difference x = x !! 1 - head x
