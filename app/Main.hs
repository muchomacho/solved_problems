module Main where

import Control.Monad
import Control.Applicative

main = do
 n <- fmap (read::String -> Int) getLine
 print $ solve n