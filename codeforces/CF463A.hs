module CF463A where

main = do
  l <- getLine
  putStr $ l ++ reverse l ++ ['\n']

