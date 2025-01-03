module LeapYear (isLeapYear) where
import Data.Predicate

somePred :: [a -> Bool] -> a -> Bool
somePred [] _ = False
somePred (f: funcs) x = f x || somePred funcs x

everyPred :: [a -> Bool] -> a -> Bool
everyPred [] _ = True
everyPred (x: xs) el = x el && everyPred xs el

reverseMod x y = mod y x
divisibleBy400 :: Integer -> Bool
divisibleBy400 x = ((== 0) . (reverseMod 400)) x

divisibleBy4andNot100 :: Integer -> Bool
divisibleBy4andNot100 =  [((/= 0) . (reverseMod 100)), ((== 0) . (reverseMod 4))] 

isLeapYear :: Integer -> Bool
-- isLeapYear year = somePred [(divisibleBy4andNot100) (divisibleBy400)] year
isLeapYear year = (divisibleBy4andNot100 year) || (divisibleBy400 year)
