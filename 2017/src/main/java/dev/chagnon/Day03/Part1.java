package dev.chagnon.Day03;

import dev.chagnon.Coordinate;
import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.*;

public class Part1 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(3);
        int address = Integer.parseInt(lines.get(0));

        Map<Integer, Set<Integer>> grid = new HashMap<>();
        Coordinate current = new Coordinate(0, 0);
        logCoordinate(grid, current);

        for (int i = 2; i <= address; i++) {
            current = nextAvailablePosition(grid, current);
            logCoordinate(grid, current);
        }

        System.out.println(Math.abs(current.getX()) + Math.abs(current.getY()));
    }

    public static void logCoordinate(Map<Integer, Set<Integer>> grid, Coordinate coordinate) {
        if (!grid.containsKey(coordinate.getX())) {
            grid.put(coordinate.getX(), new HashSet<>());
        }
        grid.get(coordinate.getX()).add(coordinate.getY());
    }

    public static Coordinate nextAvailablePosition(Map<Integer, Set<Integer>> grid, Coordinate coordinate) {
        if (grid.getOrDefault(coordinate.getX() - 1, new HashSet<>()).contains(coordinate.getY()) && !grid.getOrDefault(coordinate.getX(), new HashSet<>()).contains(coordinate.getY() + 1)) {
            return new Coordinate(coordinate.getX(), coordinate.getY() + 1);
        }
        if (grid.getOrDefault(coordinate.getX(), new HashSet<>()).contains(coordinate.getY() - 1) && !grid.getOrDefault(coordinate.getX() - 1, new HashSet<>()).contains(coordinate.getY())) {
            return new Coordinate(coordinate.getX() - 1, coordinate.getY());
        }
        if (grid.getOrDefault(coordinate.getX() + 1, new HashSet<>()).contains(coordinate.getY()) && !grid.getOrDefault(coordinate.getX(), new HashSet<>()).contains(coordinate.getY() - 1)) {
            return new Coordinate(coordinate.getX(), coordinate.getY() - 1);
        }
        return new Coordinate(coordinate.getX() + 1, coordinate.getY());
    }
}
