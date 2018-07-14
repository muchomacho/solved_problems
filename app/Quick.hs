module Quick where

quickSort:: (Ord a) => [a] -> [a]
quickSort [] = []
quickSort (x:xs) = quickSort [a | a <- xs, a <= x] ++ [x] ++ quickSort [a | a <- xs, a > x]

main = do
  let x = [6, 3, 7, 3, 7, 3, 8, 0, 2, 5, 4]
  print (quickSort x)
