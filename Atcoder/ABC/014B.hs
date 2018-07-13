module Main where

import Control.Monad
import Control.Applicative
import Data.List

main = do
 [n, x] <- map (read::String -> Int) . words <$> getLine
 let binaryX = itob x n
 values <- map (read::String -> Int) . reverse . words <$> getLine
 let answer = foldl' (\ s (val, bin) -> if bin == "1" then s + val else s) 0 $ zip values binaryX
 print answer

itob::Int -> Int -> [String]
itob x bit = let recursive _ 0 bin = bin
                 recursive x n bin = recursive (x `div` 2) (n - 1) $ show (x `mod` 2):bin
             in recursive x bit []
