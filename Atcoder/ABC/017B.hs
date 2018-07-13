module Main where

import Control.Monad
import Control.Applicative

main = do
 s <- getLine
 if choku s then putStrLn "YES" else putStrLn "NO"

choku::String -> Bool
choku [] = True
choku ('c':'h':rest) = choku rest
choku ('o':rest) = choku rest
choku ('k':rest) = choku rest
choku ('u':rest) = choku rest
choku _ = False
