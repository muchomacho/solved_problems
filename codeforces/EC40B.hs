module EC40B where

import Control.Monad
import Control.Applicative

main = do
 n <- (read::String -> Int) <$> getLine
 s <- getLine
 let max_length = maximum $ loop s n
 if max_length /= 0 then print $ n + 1 - max_length else print n

loop::String -> Int -> [Int]
loop x l = do
            end <- [0..l - 1]
            let single = take (end + 1) x
                double = take (end * 2 + 2) x
            if (single ++ single) == double
             then return (end + 1)
             else return 0
