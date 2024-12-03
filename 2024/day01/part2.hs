import Data.List
import Data.List.Split
import System.IO

main = do
  contents <- readFile "input"
  print
    . sum
    . countAinBAndMultiplyByA
    . transpose
    . map readLine
    . lines
    $ contents

readLine :: String -> [Int]
readLine =
  map read
    . splitOn "   "

count :: (Eq a) => a -> [a] -> Int
count x = length . filter (== x)

countAinBAndMultiplyByA :: [[Int]] -> [Int]
countAinBAndMultiplyByA x = map (\y -> count y (x !! 1) * y) $ head x
