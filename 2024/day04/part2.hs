import Data.Function ((&))
import Data.List.Split.Internals (fromElem)

main = do
  content <- readFile "input"
  let matrix = content & lines
  print (crossXMasCountInMatrix matrix)

matrixSize :: [String] -> (Int, Int)
matrixSize matrix = (length (head matrix), length matrix)

coordIsInMatrix :: (Int, Int) -> [String] -> Bool
coordIsInMatrix (x, y) matrix = do
  let (matX, matY) = matrixSize matrix
  (x < matX) && (y < matY) && x >= 0 && y >= 0

calcNextCoord :: (Int, Int) -> (Int, Int) -> (Int, Int)
calcNextCoord (matrixXLen, matrixYLen) (x, y)
  | (x >= matrixXLen - 2) && (y >= matrixYLen - 2) = (-1, -1)
  | x >= matrixXLen - 2 = (1, y + 1)
calcNextCoord _ (x, y) = (x + 1, y)

calcNextCoordMat :: [String] -> (Int, Int) -> (Int, Int)
calcNextCoordMat matrix = calcNextCoord (matrixSize matrix)

isCharAtCoord :: Char -> [String] -> (Int, Int) -> Bool
isCharAtCoord char matrix (x, y) = char == ((matrix !! y) !! x)

isMatchInMatrixFromCoordInDirection :: String -> [String] -> (Int, Int) -> (Int, Int) -> Bool
isMatchInMatrixFromCoordInDirection "" _ _ _ = True
isMatchInMatrixFromCoordInDirection matchString matrix (curX, curY) (dirX, dirY) = do
  let curCoord = (curX, curY)
  coordIsInMatrix curCoord matrix
    && isCharAtCoord (head matchString) matrix curCoord
    && isMatchInMatrixFromCoordInDirection (tail matchString) matrix (curX + dirX, curY + dirY) (dirX, dirY)

isMasOrSamAtRelCoordInDirection :: [String] -> (Int, Int) -> (Int, Int) -> Bool
isMasOrSamAtRelCoordInDirection matrix (x, y) (relX, relY) = do
  any (\str -> isMatchInMatrixFromCoordInDirection str matrix (x + relX, y + relY) (-relX, -relY)) ["MAS", "SAM"]

isCrossXMasAtCoord :: [String] -> (Int, Int) -> Bool
isCrossXMasAtCoord matrix coord = isMasOrSamAtRelCoordInDirection matrix coord (-1, -1) && isMasOrSamAtRelCoordInDirection matrix coord (-1, 1)

crossXMasCountInMatrixFromCoord :: [String] -> (Int, Int) -> Int
crossXMasCountInMatrixFromCoord matrix coord
  | not (coordIsInMatrix coord matrix) = 0
  | otherwise = fromEnum (isCrossXMasAtCoord matrix coord) + crossXMasCountInMatrixFromCoord matrix (calcNextCoordMat matrix coord)

crossXMasCountInMatrix :: [String] -> Int
crossXMasCountInMatrix matrix = crossXMasCountInMatrixFromCoord matrix (1, 1)
