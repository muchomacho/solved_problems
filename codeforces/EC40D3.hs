module EC40D3 where

import Control.Monad
import Control.Applicative
import Data.List
import Data.Array.Unboxed
import qualified Data.Set as S
import qualified Data.Map as M
import Data.Maybe(fromMaybe)

type Cost = Int
type Vertex = Int
type Edge = (Vertex, Vertex)

main = do
 [n, m, s, t] <- map (read:: String -> Int) . words <$> getLine
 roads <- map (tuple . sort . map (read:: String -> Int) . words) . lines <$> getContents
 print . length . possible n (s, t) $ S.fromList roads

possible:: Int -> (Vertex, Vertex) -> S.Set Edge -> [Edge]
possible n (s, g) edge = let sDist = bfs n s edge
                             gDist = bfs n g edge
                             originalTime = sDist ! g
                         in do
                             start <- [1..n - 1]
                             end <- [start + 1..n]
                             guard $ (start, end) `S.notMember` edge
                             let route1 = sDist ! start + gDist ! end + 1
                                 route2 = sDist ! end + gDist ! start + 1
                             guard $ route1 >= originalTime && route2 >= originalTime
                             return (start, end)

bfs:: Int -> Vertex -> S.Set Edge -> UArray Vertex Cost
bfs n s edge = let connect = foldl' (\ cMap (a, b) -> update a b cMap) (M.fromList [(x, []) | x <- [1..n]]) $ S.toList edge
                   queue = (s, 0):walk 1 queue connect (S.fromList [s])
               in array (1, n) queue

walk:: Int -> [(Vertex, Cost)] -> M.Map Vertex [Vertex] -> S.Set Vertex -> [(Vertex, Cost)]
walk 0 _ _ used = []
walk n ((v, c):rest) connect used = let reachable = fromMaybe (error "key error") $ M.lookup v connect
                                        unreached = filter (`S.notMember` used) reachable
                                        add = zip unreached [c + 1, c + 1..]
                                        newUsed = foldl' (flip S.insert) used unreached
                                    in add ++ walk (n + length add - 1) rest connect newUsed

update:: Vertex -> Vertex -> M.Map Vertex [Vertex] -> M.Map Vertex [Vertex]
update a b = M.update (\ x -> Just (a:x)) b . M.update (\ x -> Just (b:x)) a

tuple::[a] -> (a, a)
tuple [x, y] = (x, y)
