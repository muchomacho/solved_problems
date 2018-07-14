module EC39C where

import Data.List

main = do
  string <- getLine
  putStrLn . fst $ change ([], string) 'a'

change::(String, String) -> Char -> (String, String)
change (left, []) _ = ("-1", [])
change (left, x:xs) 'z' = if x == 'z' || x == 'y' then (left ++ "z" ++ xs, []) else change (left ++ [x], xs) 'z'
change (left, x:xs) alpha = if x == alpha || x == pred alpha then change (left ++ [alpha], xs) (succ alpha) else change (left ++ [x], xs) alpha





