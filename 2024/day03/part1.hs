import Data.Function
import Text.Regex.Posix

main :: IO ()
main = do
  content <- readFile "input"
  let matchRegex = "mul\\([0-9]{1,3}\\,[0-9]{1,3}\\)"
  let allMatches = getAllTextMatches (content =~ matchRegex) :: [String]
  print (allMatches & map calculateMulStr & sum)

getNumbersInMulStr :: String -> [Int]
getNumbersInMulStr x = map read (getAllTextMatches (x =~ "[0-9]{1,3}") :: [String])

calculateMulStr :: String -> Int
calculateMulStr = product . getNumbersInMulStr
