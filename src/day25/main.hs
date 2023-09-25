fromSnafu :: Num a => [Char] -> a
fromSnafu [] = 0
fromSnafu (x : xs) = fromSnafu' 0 (x : xs)
  where
    fromSnafu' acc [] = acc
    fromSnafu' acc [x] = acc * 5 + parseC x
    fromSnafu' acc (x : xs) = fromSnafu' (acc * 5 + parseC x) xs
    parseC '2' = 2
    parseC '1' = 1
    parseC '0' = 0
    parseC '-' = -1
    parseC '=' = -2

toSnafu :: Integral a => a -> [Char]
toSnafu n = toSnafu' "" n
  where
    toSnafu' x 0 = x
    toSnafu' soFar n = case mod n 5 of
      0 -> toSnafu' ('0' : soFar) (div n 5)
      1 -> toSnafu' ('1' : soFar) (div n 5)
      2 -> toSnafu' ('2' : soFar) (div n 5)
      3 -> toSnafu' ('=' : soFar) (div n 5 + 1)
      4 -> toSnafu' ('-' : soFar) (div n 5 + 1)

main :: IO ()
main = do
  input <- readFile "puzzle_input/day25"
  putStrLn $ toSnafu $ sum $ map fromSnafu $ lines input
