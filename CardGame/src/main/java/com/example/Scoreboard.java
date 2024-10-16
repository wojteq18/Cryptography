package com.example;
import java.util.List;

public class Scoreboard {
    private List<Player> players;

    public Scoreboard(List<Player> players) {
        this.players = players;
    }

    public void displayScores() {
        System.out.println("Current scores:");
        for (Player player : players) {
            System.out.println(player);
        }
    }

    public Player getWinner() {
        Player gameWinner = null;
        int highestScore = 0;
        for (Player player : players) {
            if (player.getScore() > highestScore) {
                highestScore = player.getScore();
                gameWinner = player;
            }
        }
        return gameWinner;
    }
}
