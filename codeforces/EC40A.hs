module EC40A where

import Control.Monad
import Control.Applicative

main = do
 n <- (read::String -> Int) <$> getLine
 s <- getLine

 print . length $ foldl trans [] s

trans::String -> Char -> String
trans ('R':rest) 'U' = 'D':rest
trans ('U':rest) 'R' = 'D':rest
trans rest x = x:rest
