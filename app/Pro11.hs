module Pro11 where

import Control.Monad
import Control.Applicative
import qualified BTree as B

main = do
  [n] <- parse . words <$> getLine
  [m] <- parse . words <$> getLine
  numbers <- parse . words <$> getLine
  let double = combi numbers
  print double
  let tree = B.toTree double
  print tree
  if any (\x -> B.existElem (m - x) tree) double
    then putStrLn "YES"
    else putStrLn "NO"

parse::[String] -> [Int]
parse = foldr (\x -> case maybeReads x of Just x -> (x:)
                                          Nothing -> id) []

maybeReads::String -> Maybe Int
maybeReads x = case reads x of [(num, rest)] -> Just num
                               _ -> Nothing

combi::[Int] -> [Int]
combi x = do
  a <- x
  b <- x
  return (a + b)
