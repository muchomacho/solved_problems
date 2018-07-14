module CF473B where

import Control.Applicative
import Control.Monad
import Data.Array.Unboxed
import Data.Array.IArray
import Data.List(foldl')
import qualified Data.Map as M
import Data.Maybe(fromMaybe)
import Data.Int

main = do
 [n, k, m] <- map (read:: String -> Int) . words <$> getLine
 keyArray <- listArray (1, n) . words <$> getLine :: IO (Array Int String)
 costArray <- listArray (1, n) . map (read:: String -> Int64) . words <$> getLine :: IO (UArray Int Int64)
 groups <- replicateM k (map (read:: String -> Int) . tail . words <$> getLine)
 contents <- words <$> getLine
 let costMap = foldl' (foldl' (\ cMap (a, b) -> M.insert a b cMap)) M.empty $ map (\ list -> let minCost = minimum $ map (costArray !) list in zip (map (keyArray !) list) [minCost,minCost..]) groups

 print . foldl' (+) 0 $ map (\w -> fromMaybe (error "key error") $ M.lookup w costMap) contents
