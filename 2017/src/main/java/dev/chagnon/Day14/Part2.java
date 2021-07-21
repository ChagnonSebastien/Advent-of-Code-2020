package dev.chagnon.Day14;

import dev.chagnon.Coordinate;
import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.*;
import java.util.stream.Collectors;

import static dev.chagnon.Day14.Part1.knotHash;

public class Part2 {

    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(14);
        String keyStringBase = lines.get(0);

        char[][] grid = new char[128][];

        for (int i = 0; i < 128; i++) {
            String keyString = keyStringBase + "-" + i;
            int [] knotHash = knotHash(keyString);
            grid[i] = Arrays.stream(knotHash)
                    .boxed()
                    .map(n -> Integer.toString(n, 2))
                    .map(s -> String.format("%8s", s))
                    .map(s -> s.replaceAll(" ", "0"))
                    .collect(Collectors.joining())
                    .toCharArray();
        }

        int amountGroups = 0;
        Set<Coordinate> inGroup = new HashSet<>();
        for (int i = 0; i < 128; i++) {
            for (int j = 0; j < 128; j++) {
                Set<Coordinate> currentGroup = new HashSet<>();
                Deque<Coordinate> potential = new ArrayDeque<>(List.of(Coordinate.from(i, j)));

                while (!potential.isEmpty()) {
                    Coordinate c = potential.pop();
                    if (grid[c.getX()][c.getY()] == '0' || inGroup.contains(c) || currentGroup.contains(c)) {
                        continue;
                    }

                    currentGroup.add(c);
                    for (Coordinate neighbor : c.getNeighbors(0, 0, 127, 127)) {
                        if (!potential.contains(neighbor) && !currentGroup.contains(neighbor)) {
                            potential.add(neighbor);
                        }
                    }
                }

                if (!currentGroup.isEmpty()) {
                    amountGroups++;
                    inGroup.addAll(currentGroup);
                }
            }
        }

        System.out.println(amountGroups);
    }
}
