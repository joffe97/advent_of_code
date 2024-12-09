import Data.Function ((&))
import Data.List (insert)
import Data.Map (Map)
import Data.Map qualified as Map
import Data.Set (Set)
import Data.Set qualified as Set

type Coord = (Int, Int)

type Matrix = [String]

type CharCoordMap = Map Char [Coord]

type CoordPair = (Coord, Coord)

main = do
  content <- readFile "input"
  let matrix = content & lines
  let matrixBounds = matrix & matrixBoundaries
  print
    ( matrix
        & createCharCoordMap
        & findAntinodesForCharCoordMap matrixBounds
        & Set.fromList
        & length
    )

fillCharCoordMapFromCharCoordList :: [(Char, Coord)] -> CharCoordMap -> CharCoordMap
fillCharCoordMapFromCharCoordList [] charCoordMap = charCoordMap
fillCharCoordMapFromCharCoordList ((char, coord) : rest) charCoordMap =
  charCoordMap
    & Map.insert char (Map.findWithDefault [] char charCoordMap & insert coord)
    & fillCharCoordMapFromCharCoordList rest

fillCharCoordMapFromString :: String -> Int -> CharCoordMap -> CharCoordMap
fillCharCoordMapFromString "" _ charCoordMap = charCoordMap
fillCharCoordMapFromString string y charCoordMap =
  charCoordMap
    & fillCharCoordMapFromCharCoordList
      ( zipWith (\char x -> (char, (y, x))) string [0 ..]
          & filter (\y -> y & fst & (/= '.'))
      )

fillCharCoordMap :: Matrix -> CharCoordMap -> CharCoordMap
fillCharCoordMap [] charCoordMap = charCoordMap
fillCharCoordMap matrix charCoordMap = (charCoordMap & fillCharCoordMapFromString (last matrix) (length matrix - 1)) & fillCharCoordMap (init matrix)

createCharCoordMap :: Matrix -> CharCoordMap
createCharCoordMap matrix = fillCharCoordMap matrix Map.empty

negCoord :: Coord -> Coord
negCoord (y, x) = (-y, -x)

addCoords :: Coord -> Coord -> Coord
addCoords (y1, x1) (y2, x2) = (y1 + y2, x1 + x2)

subCoords :: Coord -> Coord -> Coord
subCoords coord1 coord2 = addCoords coord1 (negCoord coord2)

sortCoords :: Coord -> Coord -> (Coord, Coord)
sortCoords coord1@(y1, x1) coord2@(y2, x2)
  | (y1 < y2) || ((y1 == y2) && (x1 < x2)) = (coord1, coord2)
  | otherwise = (coord2, coord1)

findAntinodesForCoordInDir :: CoordPair -> Coord -> Coord -> [Coord]
findAntinodesForCoordInDir coordPair@(firstBoundary, secondBoundary) coord@(y, x) direction@(yDir, xDir) =
  let newCoord = addCoords coord direction
   in if isWithinBoundaries firstBoundary secondBoundary newCoord then newCoord : findAntinodesForCoordInDir coordPair newCoord direction else []

findAntinodesForCharCoordsPair :: CoordPair -> Coord -> Coord -> [Coord]
findAntinodesForCharCoordsPair boundary coord1@(y1, x1) coord2@(y2, x2) =
  let (first, second) = sortCoords coord1 coord2
      diff@(yDiff, xDiff) = subCoords second first
   in [first, second] ++ findAntinodesForCoordInDir boundary second diff ++ findAntinodesForCoordInDir boundary first (negCoord diff)

pairToList :: (Coord, Coord) -> [Coord]
pairToList (a, b) = [a, b]

findAntinodesForCharCoords :: CoordPair -> [Coord] -> [Coord]
findAntinodesForCharCoords _ [x] = []
findAntinodesForCharCoords coordPair charCoords =
  (charCoords & tail & concatMap (\y -> charCoords & head & findAntinodesForCharCoordsPair coordPair y))
    ++ (charCoords & tail & findAntinodesForCharCoords coordPair)

findAntinodesForCharCoordMap :: CoordPair -> CharCoordMap -> [Coord]
findAntinodesForCharCoordMap coordPair charCoordMap = charCoordMap & Map.elems & concatMap (findAntinodesForCharCoords coordPair)

-- Order dependent
isWithinBoundaries :: Coord -> Coord -> Coord -> Bool
isWithinBoundaries first@(yf, xf) second@(ys, xs) (y, x) = yf <= y && y <= ys && xf <= x && x <= xs

matrixBoundaries :: Matrix -> (Coord, Coord)
matrixBoundaries matrix = ((0, 0), (length matrix - 1, matrix & head & length & subtract 1))
