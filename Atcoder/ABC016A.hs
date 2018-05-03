module Main where

import Control.Monad
import Control.Applicative

main = do
 [month, day] <- map (read::String -> Int) . words <$> getLine
 if month `mod` day == 0 then putStrLn "YES" else putStrLn "NO"
