module BTree (
BTree
, toTree
, insertElem
, existElem
, deleteElem
, minTree
, maxTree
)where

data BTree a = EmptyTree | Node (BTree a) a (BTree a) deriving(Show, Eq)

toTree :: (Ord a) => [a] -> BTree a
toTree = foldr insertElem EmptyTree

insertElem::(Ord a) => a -> BTree a -> BTree a
insertElem x EmptyTree = Node EmptyTree x EmptyTree
insertElem x (Node left y right)
 | x == y = Node left y right
 | x > y = Node left y (insertElem x right)
 | x < y = Node (insertElem x left) y right

existElem::(Ord a) => a -> BTree a -> Bool
existElem x EmptyTree = False
existElem x (Node left y right)
 | x == y = True
 | x > y = existElem x right
 | x < y = existElem x left

deleteElem::(Ord a) => a -> BTree a -> BTree a
deleteElem x EmptyTree = EmptyTree
deleteElem x (Node left y right)
 | x == y && left == EmptyTree && right == EmptyTree = EmptyTree
 | x == y && left == EmptyTree = right
 | x == y && right == EmptyTree = left
 | x == y = let min_right = minTree right in Node left min_right (deleteElem min_right right)
 | x > y = Node left y (deleteElem x right)
 | x < y = Node (deleteElem x left) y right

minTree::BTree a -> a
minTree (Node EmptyTree x right) = x
minTree (Node left x right) = minTree left

maxTree::BTree a -> a
maxTree (Node left x EmptyTree) = x
maxTree (Node left x right) = maxTree right