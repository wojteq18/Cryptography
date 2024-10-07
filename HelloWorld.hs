main :: IO ()
main = do
    putStrLn "Prosty kalkulator w Haskellu"
    putStrLn "Wybierz operację:"
    putStrLn "1. Dodawanie (+)"
    putStrLn "2. Odejmowanie (-)"
    putStrLn "3. Mnożenie (*)"
    putStrLn "4. Dzielenie (/)"
    putStrLn "Wpisz numer operacji: "
    operacja <- getLine
    putStrLn "Podaj pierwszą liczbę:"
    liczba1 <- getLine
    putStrLn "Podaj drugą liczbę:"
    liczba2 <- getLine
    let x = read liczba1 :: Double
    let y = read liczba2 :: Double
    let wynik = oblicz operacja x y
    putStrLn ("Wynik: " ++ show wynik)

oblicz :: String -> Double -> Double -> Double
oblicz operacja x y
    | operacja == "1" = x + y
    | operacja == "2" = x - y
    | operacja == "3" = x * y
    | operacja == "4" = if y /= 0 then x / y else error "Błąd: dzielenie przez zero!"
    | otherwise = error "Nieznana operacja"
