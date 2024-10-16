public class Team
{
    final String name;
    private int points;
    private int goalScored;
    private int goalConceded;

    public Team(String name)
    {
        this.name = name;
        this.points = 0;
        this.goalScored = 0;
        this.goalConceded = 0;
    }

    
    public String getName()
    {
        return name;
    }

    public int getPoints()
    {
        return points;
    }

    public void addMatchResult(int goalScored, int goalConceded)
    {
        this.goalScored += goalScored;
        this.goalConceded += goalConceded;
        if(goalScored > goalConceded)
        {
            points += 3;
        }
        else if(goalScored == goalConceded)
        {
            points += 1;
        }
    }
}