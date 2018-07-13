module Main where

import Control.Monad
import Control.Applicative

main = do
 [a, b, c] <- map (read::String -> Int) . words <$> getLine
 solve a b c

solve::Int -> Int -> Int -> IO ()
solve a b c
 | a + b == c && a - b == c = putStrLn "?"
 | a + b == c = putStrLn "+"
 | a - b == c = putStrLn "-"
 | otherwise = putStrLn "!"
