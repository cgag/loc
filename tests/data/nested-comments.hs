{-

{-
Nested comment
-}

-- Note: still commented
fibs :: [Int]
fibs = 1 : 1 : zipWith (+) fibs (tail fibs)

-}

main :: IO ()
main = print [1..]
