import Data.Function ((&))
import Data.List (findIndex)
import Data.List.Split (splitOn)
import Data.Map (Map, empty, findWithDefault, insertWith)
import Data.Map qualified as Map
import Data.Maybe (isNothing)

type RuleType = (Int, Int)

type PagesType = [Int]

type NumsBeforeKeyRulesMap = Map Int [Int]

main :: IO ()
main = do
  content <- readFile "input"
  let (rules, pagesList) = parseFileContent content
  let rulesMap = rules & rulesToMap
  print (pagesList & invalidPages rulesMap & map (middleOfPages rulesMap . fixPages rulesMap) & sum)

rulesToMapFill :: [RuleType] -> NumsBeforeKeyRulesMap -> NumsBeforeKeyRulesMap
rulesToMapFill [] ruleMap = ruleMap
rulesToMapFill rules ruleMap =
  let (from, to) = head rules
   in ruleMap & insertWith (++) to [from] & rulesToMapFill (tail rules)

rulesToMap :: [RuleType] -> NumsBeforeKeyRulesMap
rulesToMap rules = rulesToMapFill rules empty

parseFileContent :: String -> ([RuleType], [PagesType])
parseFileContent content = do
  let [rulesLines, pagesLines] = content & splitOn "\n\n" & map lines
  let rules = map ((\y -> (head y, y !! 1)) . (\y -> splitOn "|" y & map read)) rulesLines
  let pagesList = pagesLines & map (\y -> splitOn "," y & map read)
  (rules, pagesList)

getFirstInvalidIndexForHead :: NumsBeforeKeyRulesMap -> PagesType -> Maybe Int
getFirstInvalidIndexForHead rulesMap pages = do
  let rules = rulesMap & findWithDefault [] (head pages)
  let tailPages = tail pages
  tailPages & findIndex (`elem` rules) & fmap (+ 1)

isFirstInPagesValid :: NumsBeforeKeyRulesMap -> PagesType -> Bool
isFirstInPagesValid rulesMap pages = getFirstInvalidIndexForHead rulesMap pages & isNothing

isPagesValid :: NumsBeforeKeyRulesMap -> PagesType -> Bool
isPagesValid _ [] = True
isPagesValid rulesMap pages = do
  isFirstInPagesValid rulesMap pages && (pages & tail & isPagesValid rulesMap)

middleOfPages :: NumsBeforeKeyRulesMap -> PagesType -> Int
middleOfPages rulesMap pages = pages !! div (length pages) 2

invalidPages :: NumsBeforeKeyRulesMap -> [PagesType] -> [PagesType]
invalidPages rulesMap pages = pages & filter (not . isPagesValid rulesMap)

fixPagesForHead :: NumsBeforeKeyRulesMap -> PagesType -> PagesType
fixPagesForHead rulesMap pages = do
  let maybeIndex = getFirstInvalidIndexForHead rulesMap pages
  if maybeIndex & isNothing then pages else fixPagesForHeadWithIndex rulesMap pages maybeIndex
  where
    fixPagesForHeadWithIndex :: NumsBeforeKeyRulesMap -> PagesType -> Maybe Int -> PagesType
    fixPagesForHeadWithIndex rulesMap pages (Just index) =
      (pages !! index)
        : (pages & take index)
        ++ (pages & reverse & take ((pages & length) - index - 1) & reverse)
        & fixPagesForHead
          rulesMap
    fixPagesForHeadWithIndex _ pages Nothing = pages

fixPages :: NumsBeforeKeyRulesMap -> PagesType -> PagesType
fixPages rulesMap pages = fixPagesForHead rulesMap pages & fixNext
  where
    fixNext :: PagesType -> PagesType
    fixNext [] = []
    fixNext [x] = [x]
    fixNext pages = head pages : (pages & tail & fixPages rulesMap)
