module Common where

import Control.Applicative
import Data.Char (isSpace)
import Data.List
import GHC.IO (unsafePerformIO)
import Text.Megaparsec (ParseErrorBundle, ShowErrorComponent, TraversableStream, VisualStream, errorBundlePretty)

split :: (Eq a) => a -> [a] -> [[a]]
split _ [] = []
split onChar toSplit = before : split onChar (drop 1 after)
  where
    (before, after) = span (/= onChar) toSplit

unwrapPEB ::
    (VisualStream s, TraversableStream s, ShowErrorComponent e) =>
    Either (ParseErrorBundle s e) a ->
    Maybe a
unwrapPEB (Left e) = unsafePerformIO $ do
    putStrLn $ errorBundlePretty e
    return Nothing
unwrapPEB (Right x) = Just x

transpose' :: [[a]] -> [[a]]
transpose' = getZipList . traverse ZipList

windows :: Int -> [a] -> [[a]]
windows m = transpose' . take m . tails

trim :: String -> String
trim = f . f
  where
    f = reverse . dropWhile isSpace
