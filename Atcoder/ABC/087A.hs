module ABC087A where

main = do
  l1 <- getLine
  let x = read l1
  l2 <- getLine
  let a = read l2
  l3 <- getLine
  let b = read l3
  print (mod (x - a) b)

