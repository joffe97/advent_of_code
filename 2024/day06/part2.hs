import Data.Function ((&))
import Data.List (findIndex)
import Data.Maybe (isJust)
import Data.Set (Set)
import Data.Set qualified as Set

type Coord = (Int, Int)

type Direction = (Int, Int)

type MapMatrix = [String]

type MapData = (MapMatrix, Coord)

type GuardState = (Coord, Char)

main = do
  content <- readFile "input"
  let matrix = content & lines
  let guardCoord = matrix & findGuard
  let mapData = (matrix, guardCoord)
  let wallsToAdd = mapData & moveUntilWall & snd & Set.delete guardCoord & Set.elems
  print (wallsToAdd & map (\y -> mapData & addWall y & isCircular & fromEnum) & sum)

formatMatrix :: MapMatrix -> String
formatMatrix = foldl (\y acc -> y ++ "\n" ++ acc) ""

guardDir :: Char -> Direction
guardDir '^' = (-1, 0)
guardDir 'v' = (1, 0)
guardDir '>' = (0, 1)
guardDir '<' = (0, -1)
guardDir _ = (0, 0)

rotateChar :: Char -> Char
rotateChar '^' = '>'
rotateChar 'v' = '<'
rotateChar '>' = 'v'
rotateChar '<' = '^'

rotateGuard :: MapData -> MapData
rotateGuard (matrix, guardCoord) = (matrix & setCharInMatrix (matrix & getCharAtCoord guardCoord & rotateChar) guardCoord, guardCoord)

findGuardInLine :: String -> Maybe Int
findGuardInLine = findIndex (\y -> (y & guardDir) /= (0, 0))

findGuard :: MapMatrix -> Coord
findGuard [] = (-1, -1)
findGuard matrix = maybe (findGuard (init matrix)) (length matrix - 1,) (matrix & last & findGuardInLine)

getCharAtCoord :: Coord -> MapMatrix -> Char
getCharAtCoord (y, x) matrix = matrix !! y !! x

getCharAtCoordMaybe :: Coord -> MapMatrix -> Maybe Char
getCharAtCoordMaybe (y, x) matrix
  | (y >= 0) && (x >= 0) && (length matrix > y) && (length (matrix !! y) > x) = matrix !! y !! x & Just
  | otherwise = Nothing

isObstacle :: Coord -> MapMatrix -> Bool
isObstacle coord matrix = getCharAtCoord coord matrix == '#'

addCoords :: Coord -> Coord -> Coord
addCoords (y1, x1) (y2, x2) = (y1 + y2, x1 + x2)

setEntryInList :: a -> Int -> [a] -> [a]
setEntryInList item index list =
  let (partA, partB) = list & splitAt index
   in partA ++ item : tail partB

setCharInMatrix :: Char -> Coord -> MapMatrix -> MapMatrix
setCharInMatrix char (y, x) matrix =
  matrix & setEntryInList (matrix !! y & setEntryInList char x) y

moveGuard :: Coord -> MapData -> MapData
moveGuard toCoord (matrix, fromCoord) =
  ( matrix
      & setCharInMatrix (getCharAtCoord fromCoord matrix) toCoord
      & setCharInMatrix '.' fromCoord,
    toCoord
  )

calcFacingTile :: MapData -> Coord
calcFacingTile (matrix, guardCoord) = matrix & getCharAtCoord guardCoord & guardDir & addCoords guardCoord

moveOne :: MapData -> MapData
moveOne mapData@(matrix, _) = do
  let facingTile = calcFacingTile mapData
  let isFacingObstacle = matrix & isObstacle facingTile
  if isFacingObstacle
    then mapData & rotateGuard & moveOne
    else mapData & moveGuard facingTile

moveUntilWallWithCoordTrack :: (MapData, [Coord]) -> (MapData, [Coord])
moveUntilWallWithCoordTrack (mapData@(matrix, _), coordTrack)
  | matrix & getCharAtCoordMaybe (calcFacingTile mapData) & isJust =
      let newMapData@(_, newGuardCoord) = mapData & moveOne
       in moveUntilWallWithCoordTrack (newMapData, newGuardCoord : coordTrack)
  | otherwise = (mapData, coordTrack)

moveUntilWall :: MapData -> (MapData, Set Coord)
moveUntilWall mapData@(_, guardCoord) =
  let (newMapData, newCoordTrack) = moveUntilWallWithCoordTrack (mapData, [guardCoord])
   in (newMapData, Set.fromList newCoordTrack)

addWall :: Coord -> MapData -> MapData
addWall coord (matrix, guardCoord) = (matrix & setCharInMatrix '#' coord, guardCoord)

getGuardChar :: MapData -> Char
getGuardChar (matrix, guardCoord) = matrix & getCharAtCoord guardCoord

getFacingCharMaybe :: MapData -> Maybe Char
getFacingCharMaybe mapData@(matrix, _) = matrix & getCharAtCoordMaybe (calcFacingTile mapData)

isCircularWithPrevStates :: Set GuardState -> MapData -> Bool
isCircularWithPrevStates guardStates mapData@(matrix, guardCoord)
  | matrix & getCharAtCoordMaybe (calcFacingTile mapData) & isJust = do
      let newMapData@(newMatrix, newGuardCoord) = mapData & moveOne
      let curGuardState = (newGuardCoord, newMapData & getGuardChar)
      (guardStates & Set.member curGuardState)
        || (newMapData & isCircularWithPrevStates (guardStates & Set.insert curGuardState))
  | otherwise = False

isCircular :: MapData -> Bool
isCircular mapData = mapData & isCircularWithPrevStates Set.empty