package com.example;

public class Main
{
    public static void main(String[] args)
    {
        
        League Ekstraklasa = new League();

        Ekstraklasa.addTeam(new Team("Lech Poznan"));
        Ekstraklasa.addTeam(new Team("Rakow Czestochowa"));
        Ekstraklasa.addTeam(new Team("Cracovia Krakow"));
        Ekstraklasa.addTeam(new Team("Jagiellonia Bialystok"));
        Ekstraklasa.addTeam(new Team("Pogon Szczecin"));
        Ekstraklasa.addTeam(new Team("Legia Warszawa"));
        Ekstraklasa.addTeam(new Team("Widzew Lodz"));
        Ekstraklasa.addTeam(new Team("GKS Katowice"));
        Ekstraklasa.addTeam(new Team("Piast Gliwice"));
        Ekstraklasa.addTeam(new Team("Motor Lublin"));
        Ekstraklasa.addTeam(new Team("Zaglebie Lubin"));
        Ekstraklasa.addTeam(new Team("Gornik Zabrze"));
        Ekstraklasa.addTeam(new Team("Korona Kielce"));
        Ekstraklasa.addTeam(new Team("Stal Mielec"));
        Ekstraklasa.addTeam(new Team("Radomiak Radom"));
        Ekstraklasa.addTeam(new Team("Lechia Gdansk"));
        Ekstraklasa.addTeam(new Team("Puszcza Niepolomice"));
        Ekstraklasa.addTeam(new Team("Slask Wroclaw"));

        Ekstraklasa.simulateSeason();
        Ekstraklasa.displayLeagueTable();
        Ekstraklasa.displayMatches();
    }
}