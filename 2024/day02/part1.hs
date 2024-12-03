main = do
  print . sumTrue . map (isSafe . parseLine) . lines =<< readFile "input"

parseLine :: String -> [Int]
parseLine = map read . words

is1To3GreaterThanB :: (Int, Int) -> Bool
is1To3GreaterThanB (a, b) = (a - b <= 3) && (a - b >= 1)

isSafeIncreasing :: [Int] -> Bool
isSafeIncreasing [x] = True
isSafeIncreasing [x, y] = is1To3GreaterThanB (y, x)
isSafeIncreasing (a : b : c) = isSafeIncreasing (b : c) && isSafeIncreasing [a, b]

isSafeDecreasing :: [Int] -> Bool
isSafeDecreasing xs = isSafeIncreasing (reverse xs)

isSafe :: [Int] -> Bool
isSafe xs = isSafeIncreasing xs || isSafeDecreasing xs

sumTrue :: [Bool] -> Int
sumTrue = length . filter id
