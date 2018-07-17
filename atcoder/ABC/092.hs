module ABC092 where

import Control.Monad
import Control.Applicative

main = do
  [a, b, c, d] <- map (read:: String -> Int) . lines <$> getContents
  print $ min a b + min c d

