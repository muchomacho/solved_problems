module Main where

main = do
 a <- getLine
 b <- getLine
 if length a >= length b then putStrLn a else putStrLn b
