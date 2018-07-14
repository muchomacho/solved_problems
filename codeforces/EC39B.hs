module EC39B where

import Control.Applicative

main = do
  [a, b] <- fmap (read::String -> Integer) . words <$> getLine
  putStrLn . unwords . map show $ altsub a b

altsub::Integer -> Integer -> [Integer]
altsub 0 b = [0, b]
altsub a 0 = [a, 0]
altsub a b
 | a >= 2 * b = altsub (a `mod` (2 * b)) b
 | b >= 2 * a = altsub a (b `mod` (2 * a))
 | otherwise = [a, b]

