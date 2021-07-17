package dev.chagnon.Day03;

import dev.chagnon.Coordinate;
import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Part2 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(3);
        int threshold = Integer.parseInt(lines.get(0));

        Map<Integer, Map<Integer, Integer>> grid = new HashMap<>();
        Coordinate current = new Coordinate(0, 0);
        logCoordinate(grid, current);

        int newValue;
        do {
            current = nextAvailablePosition(grid, current);
            newValue = logCoordinate(grid, current);
        } while (newValue <= threshold);

        System.out.println(newValue);
    }

    public static int logCoordinate(Map<Integer, Map<Integer, Integer>> grid, Coordinate coordinate) {
        int sum = Stream.of(
                new Coordinate(-1, 1), new Coordinate(0, 1), new Coordinate(1, 1),
                new Coordinate(-1, 0),                               new Coordinate(1, 0),
                new Coordinate(-1, -1), new Coordinate(0, -1), new Coordinate(1, -1)
        )
                .map(neighbor -> grid.getOrDefault(coordinate.getX() + neighbor.getX(), new HashMap<>()).getOrDefault(coordinate.getY() + neighbor.getY(), 0))
                .reduce(0, Integer::sum);

        if (sum == 0) {
            sum = 1;
        }

        if (!grid.containsKey(coordinate.getX())) {
            grid.put(coordinate.getX(), new HashMap<>());
        }
        grid.get(coordinate.getX()).put(coordinate.getY(), sum);
        return sum;
    }

    public static Coordinate nextAvailablePosition(Map<Integer, Map<Integer, Integer>> grid, Coordinate coordinate) {
        if (grid.getOrDefault(coordinate.getX() - 1, new HashMap<>()).containsKey(coordinate.getY()) && !grid.getOrDefault(coordinate.getX(), new HashMap<>()).containsKey(coordinate.getY() + 1)) {
            return new Coordinate(coordinate.getX(), coordinate.getY() + 1);
        }
        if (grid.getOrDefault(coordinate.getX(), new HashMap<>()).containsKey(coordinate.getY() - 1) && !grid.getOrDefault(coordinate.getX() - 1, new HashMap<>()).containsKey(coordinate.getY())) {
            return new Coordinate(coordinate.getX() - 1, coordinate.getY());
        }
        if (grid.getOrDefault(coordinate.getX() + 1, new HashMap<>()).containsKey(coordinate.getY()) && !grid.getOrDefault(coordinate.getX(), new HashMap<>()).containsKey(coordinate.getY() - 1)) {
            return new Coordinate(coordinate.getX(), coordinate.getY() - 1);
        }
        return new Coordinate(coordinate.getX() + 1, coordinate.getY());
    }
}
