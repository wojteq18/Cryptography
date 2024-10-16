package com.example;

public class Team
{
    private String name;
    private int points;
    private int goalsScored;
    private int goalsConceded;

    public Team(String name)
    {
        this.name = name;
        this.points = 0;
        this.goalsScored = 0;
        this.goalsConceded = 0;
    }

    public String getName()
    {
        return name;
    }

    public int getPoints()
    {
        return points;
    }

    public int getGoalsScored()
    {
        return goalsScored;
    }

    public int getGoalsConceded()
    {
        return goalsConceded;
    }

    public void addMatchResult(int scored, int conceded)
    {
        goalsScored = goalsScored + scored;
        goalsConceded = goalsConceded + conceded;
        if(scored > conceded)
        {
            points = points + 3;
        }
        else if(scored == conceded)
        {
            points = points + 1;
        }
        else
        {
            points = points + 0;
        }
    }

    public void displayTeamInfo()
    {
        System.out.println("Name: " + name + " pts: " + points + " GS: " + goalsScored + " GC: " + goalsConceded);
    }
}