module EC40D2 where

import Control.Monad
import Control.Applicative
import Data.List
import Control.Monad.ST.Safe
import Data.Array.ST.Safe
import Data.Array.Unboxed
import qualified Data.Set as S
import qualified Data.Map as M

type Cost = Int
type Vertex = Int
type Edge = ((Vertex, Vertex), Cost)

main = do
 [n, m, s, t] <- map (read:: String -> Int) . words <$> getLine
 roads <- map (tuple . sort . map (read:: String -> Int) . words) . lines <$> getContents
 print . length . possible n (s, t) . S.fromList $ zip roads [1,1..]

possible:: Int -> (Vertex, Vertex) -> S.Set Edge -> [Edge]
possible n (s, g) edge = let sDist = dijkstra n s edge
                             gDist = dijkstra n g edge
                             originalTime = sDist ! g
                         in do
                             start <- [1..n - 1]
                             end <- [start + 1..n]
                             guard $ ((start, end), 1) `S.notMember` edge
                             let route1 = sDist ! start + gDist ! end + 1
                                 route2 = sDist ! end + gDist ! start + 1
                             guard $ route1 >= originalTime && route2 >= originalTime
                             return ((start, end), 1)

dijkstra::Int -> Vertex -> S.Set Edge -> UArray Vertex Cost
dijkstra n s edge = let initialArray = runSTUArray $ do
                                                      mArray <- newArray (1, n) maxBound :: ST s (STUArray s Int Int)
                                                      let reachable = S.toList . omit s $ S.filter (\ ((a, b), c) -> a == s || b == s) edge
                                                      forM_ reachable $ uncurry (writeArray mArray)
                                                      return mArray
                        initialSet = S.delete (maxBound::Int, s) . S.fromList . map (\ (a, b) -> (b, a)) $ assocs initialArray
                        initialMap = M.delete s . M.fromList $ assocs initialArray
                    in array (1, n) . M.toList $ recursive s edge initialSet initialMap M.empty

recursive::Vertex -> S.Set Edge -> S.Set (Cost, Vertex) -> M.Map Vertex Cost -> M.Map Vertex Cost -> M.Map Vertex Cost
recursive s edge costSet costMap distMap
 | S.null costSet = M.insert s 0 distMap
 | otherwise = let (minCost, minVertex) = S.findMin costSet
                   newDistMap = M.insert minVertex minCost distMap
                   neighbors = S.toList . omit minVertex $ S.filter (\ ((a, b), c) -> a == minVertex || b == minVertex) edge
                   (newCostSet, newCostMap) = foldl' (\ (cSet, cMap) (a, c) -> case M.lookup a cMap of
                                                                                Just oldCost -> if minCost + c < oldCost
                                                                                              then (S.insert (minCost + c, a) $ S.delete (oldCost, a) cSet, M.update (\_ -> Just $ minCost + c) a cMap)
                                                                                              else (cSet, cMap)
                                                                                Nothing -> (cSet, cMap))
                                               (S.delete (minCost, minVertex) costSet, M.delete minVertex costMap) neighbors
               in recursive s edge newCostSet newCostMap newDistMap

omit::Vertex -> S.Set Edge -> S.Set (Vertex, Cost)
omit x = S.map (\ ((a, b), c) -> if a == x then (b, c) else (a, c))

tuple::[a] -> (a, a)
tuple [x, y] = (x, y)
