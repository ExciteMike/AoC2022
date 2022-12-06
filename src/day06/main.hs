import Data.List (foldl')

allUniq :: (Eq a) => [a] -> Bool
allUniq [] = True
allUniq (x : xs) = x `notElem` xs && allUniq xs

fst3 :: (a, b, c) -> a
fst3 (x, _, _) = x

windows :: Int -> String -> [String]
windows n s =
  fst3 $ foldl' f ([], 0, "") s
  where
    f :: ([String], Int, String) -> Char -> ([String], Int, String)
    f (wins, len, prev) c
      | len < n = (wins, len + 1, prev ++ [c])
      | otherwise = (wins ++ [prev], len, drop 1 prev ++ [c])

detect n s =
  case filter (allUniq . snd) $ zip [0 ..] $ windows n s of
    ((i, _) : _) -> i
    _ -> error "empty list"

main = do
  input <- readFile "puzzle_input/day06"
  putStrLn $ (++) "part 1: " $ show $ detect 4 input
  putStrLn $ (++) "part 2: " $ show $ detect 14 input