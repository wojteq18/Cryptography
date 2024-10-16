package com.example;
import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        // Inicjalizacja graczy i talii
        List<Player> players = new ArrayList<>();
        players.add(new Player("Alice"));
        players.add(new Player("Bob"));
        Deck deck = new Deck();

        // Inicjalizacja tabeli wyników
        Scoreboard scoreboard = new Scoreboard(players);

        // Przeprowadzenie kilku rund gry
        for (int round = 1; round <= 5; round++) {
            System.out.println("\nRound " + round);
            GameRound gameRound = new GameRound(players, deck);
            gameRound.startRound();
            Player roundWinner = gameRound.determineRoundWinner();
            gameRound.distributePoints(roundWinner);
            scoreboard.displayScores();
        }

        // Wyłonienie zwycięzcy gry
        Player gameWinner = scoreboard.getWinner();
        if (gameWinner != null) {
            System.out.println("\nThe winner of the game is: " + gameWinner.getName());
        } else {
            System.out.println("\nThe game ended in a draw.");
        }
    }
}
