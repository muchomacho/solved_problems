module Main where

import Control.Monad
import Control.Applicative

main = do
 problems <- map (map (read::String -> Int) . words) <$> replicateM 3 getLine
 print $ foldl (\ acc x -> acc + (product x `div` 10)) 0 problems
