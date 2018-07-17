module ABC087B where

main = do
  l1 <- getLine
  let a = read l1
  l2 <- getLine
  let b = read l2
  l3 <- getLine
  let c = read l3
  l4 <- getLine
  let x = read l4

  print (length [(s, t, u)| s <- [0..a], t <- [0..b], u <- [0..c], 500 * s + 100 * t + 50 * u == x])
