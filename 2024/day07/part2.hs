import Control.Arrow ((>>>))
import Data.Function ((&))
import Data.List.Split (splitOn)

type PuzzleEntry = (Int, [Int])

main = do
  content <- readFile "input"
  print (content & parseContent & map answerIfIsValidEntry & sum)

parseContent :: String -> [PuzzleEntry]
parseContent content = content & lines & map (splitOn ": " >>> (\y -> (head y & read, last y & splitOn " " & map read)))

combineNumbers :: Int -> Int -> Int
combineNumbers a b = (show a ++ show b) & read

isValidEntryWithFunc :: PuzzleEntry -> (Int -> Int) -> Bool
isValidEntryWithFunc (_, []) _ = False
isValidEntryWithFunc (answer, [x]) calcFunc = answer == (x & calcFunc)
isValidEntryWithFunc entry@(answer, numbers) calcFunc =
  answer == head numbers
    || (answer > head numbers)
      && ( [combineNumbers, (*), (+)]
             & any (\sign -> isValidEntryWithFunc (answer, tail numbers) (\y -> y & sign (head numbers & calcFunc)))
         )

isValidEntry :: PuzzleEntry -> Bool
isValidEntry entry = isValidEntryWithFunc entry id

answerIfIsValidEntry :: PuzzleEntry -> Int
answerIfIsValidEntry entry@(answer, _) = if isValidEntry entry then answer else 0
