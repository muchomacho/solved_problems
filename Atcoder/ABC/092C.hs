module ABC092C where

import Control.Monad
import Control.Applicative
import Data.List

main = do
 n <- (read:: String -> Int) <$> getLine
 x <- map (read:: String -> Int) . words <$> getLine
 forM_ (count n (x ++ [0])) print

count:: Int -> [Int] -> [Int]
count n x = let dist = fst $ foldl' (\(line, former) x -> ((x - former):line, x)) ([], 0) x
                comp = foldl' (+) 0 $ map abs dist
            in fst $ foldl' (\(result, d1:d2:rest) x -> ((comp - abs d1 - abs d2 + abs (d1 + d2)):result, d2:rest)) ([], dist) [0..n-1]
