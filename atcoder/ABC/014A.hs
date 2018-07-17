module Main where

main = do
 a <- fmap (read::String -> Int) getLine
 b <- fmap (read::String -> Int) getLine
 case a `mod` b of
  0 -> print 0
  x -> print $ b - x
