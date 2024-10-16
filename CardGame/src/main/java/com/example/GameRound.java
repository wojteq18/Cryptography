package com.example;
import java.util.List;

public class GameRound {
    private List<Player> players;
    private Deck deck;

    public GameRound(List<Player> players, Deck deck) {
        this.players = players;
        this.deck = deck;
    }

    public void startRound() {
        System.out.println("Starting a new round...");

        // Każdy gracz dobiera jedną kartę
        for (Player player : players) {
            Card drawnCard = deck.drawCard();
            if (drawnCard != null) {
                player.addCardToHand(drawnCard);
                System.out.println(player.getName() + " drew " + drawnCard);
            }
        }
    }

    public Player determineRoundWinner() {
        Player roundWinner = null;
        int highestValue = 0;

        for (Player player : players) {
            Card playedCard = player.playCard();
            if (playedCard != null) {
                int cardValue = playedCard.getValue();
                System.out.println(player.getName() + " played " + playedCard + " (Value: " + cardValue + ")");
                if (cardValue > highestValue) {
                    highestValue = cardValue;
                    roundWinner = player;
                }
            }
        }

        if (roundWinner != null) {
            System.out.println("Round winner: " + roundWinner.getName());
        } else {
            System.out.println("No winner this round.");
        }
        return roundWinner;
    }

    public void distributePoints(Player winner) {
        if (winner != null) {
            winner.addPoints(1);
            System.out.println(winner.getName() + " receives 1 point.");
        }
    }
}
