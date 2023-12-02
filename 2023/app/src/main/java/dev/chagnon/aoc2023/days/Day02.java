/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package dev.chagnon.aoc2023.days;

import java.util.List;
import dev.chagnon.aoc2023.DayRunner;
import lombok.ToString;

public class Day02 implements DayRunner {

    @ToString
    private static class GameResult {
        private int gameId;
        private int red = 0;
        private int green = 0;
        private int blue = 0;

        GameResult(String line) {
            var game_attempts = line.split(": ");
            gameId = Integer.parseInt(game_attempts[0].split(" ")[1]);
            var grabs = game_attempts[1].split("; ");

            for (var grab : grabs) {
                String[] components = grab.replaceAll(",", "").split(" ");

                for (int i = 0; i < components.length; i += 2) {
                    int amount = Integer.parseInt(components[i]);
                    String color = components[i + 1];
                    switch (color) {
                        case "red" -> {
                            if (amount > red)
                                red = amount;
                        }
                        case "green" -> {
                            if (amount > green)
                                green = amount;
                        }
                        case "blue" -> {
                            if (amount > blue)
                                blue = amount;
                        }
                    }
                }
            }
        }
    }

    @Override
    public String part1(String input) {
        var lines = List.of(input.split("\n"));
        var sum = 0;
        for (String line : lines) {
            var gameResult = new GameResult(line);
            if (gameResult.red > 12)
                continue;
            if (gameResult.green > 13)
                continue;
            if (gameResult.blue > 14)
                continue;
            sum += gameResult.gameId;
        }
        return String.valueOf(sum);
    }

    @Override
    public String part2(String input) {
        var lines = List.of(input.split("\n"));
        var sum = 0;
        for (String line : lines) {
            var gameResult = new GameResult(line);
            sum += gameResult.red * gameResult.green * gameResult.blue;
        }
        return String.valueOf(sum);
    }

}
