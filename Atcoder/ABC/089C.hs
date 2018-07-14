module ABC089C where

import Data.List

main = do
  n <- fmap (read::String -> Int) getLine
  names <- fmap lines getContents
  let subgroup = map (count names) "MARCH"
  print $ combi 3 subgroup

count::[String] -> Char -> Int
count x a = length . filter (\(x:xs) -> x == a) $ x

combi::Int -> [Int] -> Int
combi 0 _ = 1
combi _ [] = 0
combi n (x:xs)
 | length xs < n - 1 = 0
 | otherwise =  x * combi (n - 1) xs + combi n xs


