import Data.Function ((&))

main = do
  content <- readFile "input"
  let matrix = content & lines
  print ("XMAS" `stringCountInMatrix` matrix)

matrixSize :: [String] -> (Int, Int)
matrixSize matrix = (length (head matrix), length matrix)

coordIsInMatrix :: (Int, Int) -> [String] -> Bool
coordIsInMatrix (x, y) matrix = do
  let (matX, matY) = matrixSize matrix
  (x < matX) && (y < matY) && x >= 0 && y >= 0

calcNextCoord :: (Int, Int) -> (Int, Int) -> (Int, Int)
calcNextCoord (matrixXLen, matrixYLen) (x, y)
  | (x >= matrixXLen - 1) && (y >= matrixYLen - 1) = (-1, -1)
  | x >= matrixXLen - 1 = (0, y + 1)
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

stringCountInMatrixAtCoord :: String -> [String] -> (Int, Int) -> Int
stringCountInMatrixAtCoord matchString matrix coord = do
  [ (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1)
    ]
    & map (fromEnum . isMatchInMatrixFromCoordInDirection matchString matrix coord)
    & sum

stringCountInMatrixFromCoord :: String -> [String] -> (Int, Int) -> Int
stringCountInMatrixFromCoord matchString matrix coord
  | not (coordIsInMatrix coord matrix) = 0
  | otherwise =
      stringCountInMatrixAtCoord matchString matrix coord
        + stringCountInMatrixFromCoord matchString matrix (calcNextCoordMat matrix coord)

stringCountInMatrix :: String -> [String] -> Int
stringCountInMatrix matchString matrix = stringCountInMatrixFromCoord matchString matrix (0, 0)
