module CF473A where

main = do
 n <- fmap (read:: String -> Int) getLine
 if odd n then putStrLn "Ehab" else putStrLn "Mahmoud"

