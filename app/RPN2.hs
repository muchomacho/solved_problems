{-# LANGUAGE DeriveFunctor #-}
{-# LANGUAGE FlexibleInstances #-}
{-# LANGUAGE UndecidableInstances #-}

module RPN2 where

import Control.Monad
import Data.String
import Control.Applicative

data Either' a b = Left' a | Right' b
    deriving (Functor, Show)

instance Applicative (Either' a) where
    pure = Right'

    Left' a <*> _ = Left' a
    Right' _ <*> Left' a = Left' a
    Right' f <*> Right' x = Right' (f x)

instance IsString a => Monad (Either' a) where
    return = pure
    (>>) = (*>)

    Left' e >>= _ = Left' e
    Right' x >>= f = f x

    fail = Left' . fromString

main = do
 content <- words <$> getLine
 case solveRPN content of Right' x -> print x
                          Left' msg -> putStrLn $ msg ++ ". Suck my dick."

solveRPN::[String] -> Either' String Double
solveRPN x = do
 [result] <- foldM calculate [] x
 return result :: Either' String Double

calculate::[Double] -> String -> Either' String [Double]
calculate (a:b:rest) "+" = Right'  $ (a + b):rest
calculate (a:b:rest) "-" = Right'  $ (b - a):rest
calculate (a:b:rest) "/" = Right'  $ (b / a):rest
calculate (a:b:rest) "*" = Right'  $ (a * b):rest
calculate (a:rest) "log" = Right'  $ log a:rest
calculate rest str = case maybeReads str of Just num -> Right'  $ num:rest
                                            Nothing -> Left' $ "Error occurred at " ++ str

maybeReads::String -> Maybe Double
maybeReads x = case reads x of [(num, "")] -> Just num
                               _ -> Nothing