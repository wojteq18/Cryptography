package com.example;
import java.util.Random;

public class Match
{
    private Team homeTeam;
    private Team awayTeam;
    private int homeGoals;
    private int awayGoals;

    public Match(Team homeTeam, Team awayTeam)
    {
        this.homeTeam = homeTeam;
        this.awayTeam = awayTeam;
    }

    public void playMatch()
    {
        Random random = new Random();
        homeGoals = random.nextInt(6);
        awayGoals = random.nextInt(6);

        homeTeam.addMatchResult(homeGoals, awayGoals);
        awayTeam.addMatchResult(awayGoals, homeGoals);
    }

    public void displayMatchResult()
    {
        System.out.println(homeTeam.getName() + " " + homeGoals + " - " + awayGoals + " " + awayTeam.getName());
    }
}