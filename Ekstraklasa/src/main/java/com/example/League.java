package com.example;
import java.util.ArrayList;
import java.util.List;

public class League
{
    private List<Team> teams;
    private List<Match> matches;

    public League()
    {
        teams = new ArrayList<>();
        matches = new ArrayList<>();
    }

    public void addTeam(Team team)
    {
        teams.add(team);
    }

    public void simulateSeason()
    {
        for(int i = 0; i < teams.size(); i++)
        {
            Team homeTeam = teams.get(i);
            for(int j = i + 1; j < teams.size(); j++)
            {
                Team awayTeam = teams.get(j);
                //mecz jako gospodarz
                Match match1 = new Match(homeTeam, awayTeam);
                match1.playMatch();
                matches.add(match1);

                //mecz jako gość
                Match match2 = new Match(awayTeam, homeTeam);
                match2.playMatch();
                matches.add(match2);
            }
        }
    }

    public void displayLeagueTable()
    {
        teams.sort((t1, t2) -> 
        { //sortowanie listy drużyn
            int result = t2.getPoints() - t1.getPoints();
            if(result != 0)
            {
                return result;
            }
            
            else
            {
                int goalDifference1 = t1.getGoalsScored() - t1.getGoalsConceded();
                int goalDifference2 = t2.getGoalsScored() - t2.getGoalsConceded();
                int result2 = goalDifference1 - goalDifference2;
                return result2;
            }
        });

        for(int i = 0; i < teams.size(); i++)
        {
            System.out.println((i + 1) + ". ");
            teams.get(i).displayTeamInfo();
        }
    }

    public void displayMatches()
    {
        for(Match match : matches)
        {
            match.displayMatchResult();
        }
    }
}