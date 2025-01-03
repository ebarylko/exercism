module LeapYear (isLeapYear) where
import Control.Applicative (liftA2)

isZero = (== 0)

modBy = flip mod 
divisibleBy x y = isZero $ modBy x y

(<&&>) = liftA2 (&&)

(<||>) = liftA2 (||)
isLeapYear :: Integer -> Bool
isLeapYear =  divisibleBy4andNot100  <||>  (divisibleBy 400)
  where
    divisibleBy4andNot100 =  (not .  (divisibleBy 100))  <&&>  (divisibleBy 4)
