import Data.List (foldl')
import qualified Data.Set as Set

steps :: String -> [(Int, Int)]
steps s = replicate dist' direction'
  where
    ws = words s
    (direction : dist : _) = ws
    dist' = read dist
    direction' = case direction of
      "U" -> (0, 1)
      "D" -> (0, -1)
      "L" -> (-1, 0)
      "R" -> (1, 0)
      _ -> error "bad direction"

unique :: Ord a => [a] -> [a]
unique values = Set.toList $ Set.fromList values

sign :: (Num a, Num p, Ord a) => a -> p
sign 0 = 0
sign x
  | x > 0 = 1
  | otherwise = -1

follow :: [(Int, Int)] -> [(Int, Int)]
follow path = let (path', _, _) = foldl' followStep ([(0, 0)], 0, 0) path in path'

followStep :: ([(Int, Int)], Int, Int) -> (Int, Int) -> ([(Int, Int)], Int, Int)
followStep (path, tx, ty) (hx, hy) =
  let (dx, dy) = (hx - tx, hy - ty)
   in if (-1 <= dx) && (dx <= 1) && (-1 <= dy) && (dy <= 1)
        then (path, tx, ty)
        else
          let tx' = tx + sign dx
              ty' = ty + sign dy
           in (path ++ [(tx', ty')], tx', ty')

main :: IO ()
main = do
  input <- readFile "puzzle_input/day09"
  let path1 = follow $ reverse $ foldl' takeStep [] $ concatMap steps $ lines input
  putStrLn $ (++) "part 1: " $ show $ length $ unique path1
  let path9 = iterate follow path1 !! 8
  putStrLn $ (++) "part 2: " $ show $ length $ unique path9
  where
    takeStep :: [(Int, Int)] -> (Int, Int) -> [(Int, Int)]
    takeStep ((x, y) : rest) (dx, dy) = (x + dx, y + dy) : (x, y) : rest
    takeStep [] (dx, dy) = [(dx, dy), (0, 0)]