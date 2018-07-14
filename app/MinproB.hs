module MinproB where

import Control.Monad
import Control.Applicative

main = do
  [x,k] <- map read . words <$> getLine
  let upper = div x (10 ^ k)
  print ((upper + 1) * (10 ^ k))
