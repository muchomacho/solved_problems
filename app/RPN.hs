module RPN where

import Data.List

main = do
  problem <- fmap words getLine
  print $ solveRPN problem

solveRPN::[String] -> Double
solveRPN problem = let calculate (x:y:xs) "+" = (x + y):xs
                       calculate (x:y:xs) "-" = (y - x):xs
                       calculate (x:y:xs) "*" = (x * y):xs
                       calculate (x:y:xs) "/" = (y / x):xs
                       calculate x n = read n:x
                   in head $ foldl' calculate [] problem

