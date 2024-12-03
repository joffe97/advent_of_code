main = do
  print . sumTrue . map (isSafe . parseLine) . lines =<< readFile "input"

parseLine :: String -> [Int]
parseLine = map read . words

is1To3GreaterThanB :: (Int, Int) -> Bool
is1To3GreaterThanB (a, b) = (a - b <= 3) && (a - b >= 1)

isSafeIncreasing :: [Int] -> Int
isSafeIncreasing [x] = 0
isSafeIncreasing [x, y] = if is1To3GreaterThanB (y, x) then 0 else 1
isSafeIncreasing (a : b : c) = if isSafeIncreasing [a, b] == 0 then isSafeIncreasing (b : c) else 1 + isSafeIncreasing (a : c)

isSafeDecreasing :: [Int] -> Int
isSafeDecreasing [x] = 0
isSafeDecreasing [x, y] = if is1To3GreaterThanB (x, y) then 0 else 1
isSafeDecreasing (a : b : c) = if isSafeDecreasing [a, b] == 0 then isSafeDecreasing (b : c) else 1 + isSafeDecreasing (a : c)

isSafe :: [Int] -> Bool
isSafe xs =
  isSafeIncreasing xs <= 1
    || isSafeDecreasing xs <= 1
    || isSafeIncreasing (reverse xs) <= 1
    || isSafeDecreasing (reverse xs) <= 1

sumTrue :: [Bool] -> Int
sumTrue = length . filter id
