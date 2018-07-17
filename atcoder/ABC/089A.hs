module ABC089A where

main = do
  num <- fmap read getLine
  print $ div num 3
