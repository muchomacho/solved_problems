module EC39A where

import Control.Applicative

main = do
  n <- fmap (read::String -> Int) getLine
  seq <- map (read::String -> Int) . words <$> getLine
  print $ foldl (\x y -> if y > 0 then x + y else x - y) 0 seq

