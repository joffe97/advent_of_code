import Data.Function ((&))
import Data.List.Split (splitOn)
import Data.Map (Map, empty, findWithDefault, insertWith)
import Data.Map qualified as Map

type RuleType = (Int, Int)

type PagesType = [Int]

type NumsBeforeKeyRulesMap = Map Int [Int]

main :: IO ()
main = do
  content <- readFile "input"
  let (rules, pagesList) = parseFileContent content
  let rulesMap = rules & rulesToMap
  print (pagesList & map (middleOfValidPages rulesMap) & sum)

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
  let rules = (rulesLines & map (\y -> splitOn "|" y & map read) :: [[Int]]) & map (\y -> (head y, y !! 1))
  let pagesList = pagesLines & map (\y -> splitOn "," y & map read)
  (rules, pagesList)

isPagesValid :: NumsBeforeKeyRulesMap -> PagesType -> Bool
isPagesValid _ [] = True
isPagesValid rulesMap pages = do
  let rules = rulesMap & findWithDefault [] (head pages)
  let tailPages = tail pages
  (tailPages & any (\y -> rules & elem y) & not) && (tailPages & isPagesValid rulesMap)

middleOfValidPages :: NumsBeforeKeyRulesMap -> PagesType -> Int
middleOfValidPages rulesMap pages
  | isPagesValid rulesMap pages = pages !! div (length pages) 2
  | otherwise = 0
