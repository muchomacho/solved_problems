module ABC089B where

main = do
  n <- fmap (read:: String -> Int) getLine
  colors <- fmap words getLine
  if "Y" `elem` colors
  then putStrLn "Four"
  else putStrLn "Three"
