module ABC092B where

import Data.Array.MArray
import Control.Monad
import Control.Applicative
import Data.List
import Control.Monad.ST
import Data.Array.ST
import Data.Array.Unboxed
import qualified Data.Set as S

main = do
 n <- (read:: String -> Int) <$> getLine
 [d, x] <- map (read:: String -> Int) . words <$> getLine
 a <- map (read:: String -> Int) . lines <$> getContents
 print $ count d x a

count:: Int -> Int -> [Int] -> Int
count d x a = let choco = do
                           interval <- a
                           [1, (interval + 1)..d]
              in x + length choco

