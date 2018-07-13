module Main where

import Control.Applicative

main = do
 n <- fmap (read::String -> Int) getLine
 bugs <- map (read::String -> Int) . words <$> getLine
 let (val, num) = foldl (\ (s, num) x -> if x > 0 then (s + x, num + 1) else (s, num)) (0, 0) bugs
 if val `mod` num == 0 then print $ val `div` num else print $ val `div` num + 1
