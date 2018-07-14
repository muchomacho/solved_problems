module Shortest where

import Data.List

data Path = Path {up::Int, down::Int, cross::Int} deriving(Show)

type Route = String
type Time = Int


main = do
  pathway <- fmap (group3 . map read . lines) getContents
  let optimalPath = foldl' shortestPath (("", 0), ("", 0)) pathway
  if snd (fst optimalPath) <= snd (snd optimalPath)
  then do
    putStrLn . reverse . fst $ fst optimalPath
    print . snd $ fst optimalPath
  else do
    putStrLn . reverse . fst $ snd optimalPath
    print . snd $ snd optimalPath


group3::[Int]->[Path]
group3 [] = []
group3 (x:y:z:rest) = Path {up=x, down=y, cross=z}:group3 rest

shortestPath::((Route, Time), (Route, Time)) -> Path -> ((Route, Time), (Route, Time))
shortestPath acc x
  | upstraight <= upcross && downstraight <= downcross = append (("A", up x), ("B", down x)) acc
  | upstraight <= upcross && downstraight > downcross = append (("A", up x), ("CA", up x + cross x)) (fst acc, fst acc)
  | upstraight > upcross && downstraight <= downcross = append (("CB", down x + cross x), ("B", down x)) (snd acc, snd acc)
  where upstraight = snd (fst acc) + up x
        downstraight = snd (snd acc) + down x
        upcross = snd (snd acc) + down x + cross x
        downcross = snd (fst acc) + up x + cross x

append::((Route, Time), (Route, Time)) -> ((Route, Time), (Route, Time)) -> ((Route, Time), (Route, Time))
append x y = ((fst (fst x) ++ fst (fst y), snd (fst x) + snd (fst y)), (fst (snd x) ++ fst (snd y), snd (snd x) + snd (snd y)))
