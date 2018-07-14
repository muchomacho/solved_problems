module EC40D where

import Control.Monad
import Control.Applicative
import Data.List
import Control.Monad.ST.Safe
import Data.Array.ST.Safe
import Data.Array.Unboxed
import qualified Data.Set as S
import qualified Data.Map as M
import Data.Maybe(fromMaybe)

type Cost = Int
type Vertex = Int
type Edge = ((Vertex, Vertex), Cost)

main = do
 [n, m, s, t] <- map (read:: String -> Int) . words <$> getLine
 roads <- map (tuple . sort . map (read:: String -> Int) . words) . lines <$> getContents
 print . length . possible n (s, t) . S.fromList $ zip roads [1, 1..]

possible:: Int -> (Vertex, Vertex) -> S.Set Edge -> [Edge]
possible n (s, g) edge = let sDist = dijkstra n s edge
                             gDist = dijkstra n g edge
                             originalTime = sDist ! g
                         in
                          do
                           l <- [1..n - 1]
                           r <- [l + 1..n]
                           guard $ ((l, r), 1) `S.notMember` edge
                           let route1 = sDist ! l + gDist ! r + 1
                               route2 = sDist ! r + gDist ! l + 1
                           guard $ min route1 route2 >= originalTime
                           return ((l, r), 1)

dijkstra::Int -> Vertex -> S.Set Edge -> UArray Vertex Cost
dijkstra n s edge = let connect = foldl' (\ cMap ((a, b), c) -> insertList a b c cMap) (M.fromList [(x, []) | x <- [1..n]]) $ S.toList edge
                        costArray = runSTUArray $ do
                                                   mArray <- newArray (1, n) maxBound :: ST s (STUArray s Vertex Cost)
                                                   let reachable = fromMaybe (error "key error") $ M.lookup s connect
                                                   forM_ reachable $ uncurry (writeArray mArray)
                                                   return mArray
                        costSet = S.delete (maxBound::Int, s) . S.fromList . map (\ (a, b) -> (b, a)) $ assocs costArray
                     in runSTUArray $ do
                                       mArray <- thaw costArray
                                       loop s connect costSet mArray
                                       return mArray

loop:: Vertex -> M.Map Vertex [(Vertex, Cost)] -> S.Set (Cost, Vertex) -> STUArray s Vertex Cost -> ST s ()
loop s connect costSet costArray = if S.null costSet then writeArray costArray s 0
                                                     else do
                                                           let (minCost, minVertex) = S.findMin costSet
                                                               reachable = fromMaybe (error "key error") $ M.lookup minVertex connect
                                                           newCostSet <- foldM (\ cSet (v, c) -> do
                                                                                                  let newCost = minCost + c
                                                                                                  oldCost <- readArray costArray v
                                                                                                  if newCost < oldCost then do
                                                                                                                             writeArray costArray v newCost
                                                                                                                             return $ S.insert (newCost, v) $ S.delete (oldCost, v) cSet
                                                                                                                       else return cSet) (S.deleteMin costSet) reachable
                                                           loop s connect newCostSet costArray

insertList:: Vertex -> Vertex -> Cost -> M.Map Vertex [(Vertex, Cost)] -> M.Map Vertex [(Vertex, Cost)]
insertList a b c = M.update (\ x -> Just $ (a, c):x) b . M.update (\x -> Just $ (b, c):x) a

tuple::[a] -> (a, a)
tuple [x, y] = (x, y)
