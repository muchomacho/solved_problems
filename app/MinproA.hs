module MinproA where

judge :: String -> String
judge ('y':'a':'h':a:b:[]) = if a == b then "YES\n" else "NO\n"
judge x = "NO\n"

main = do
  l <- getLine
  putStr (judge l)

