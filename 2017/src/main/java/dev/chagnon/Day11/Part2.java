package dev.chagnon.Day11;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.Arrays;
import java.util.List;

public class Part2 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(11);

        List<String> directions = Arrays.asList(lines.get(0).split(","));
        int i = 0, j = 0, furthest = 0;

        //    | n | ne
        // ---+---+---
        // nw |   | se
        // ---+---+---
        // sw | s |

        for (String d : directions) {
            switch (d) {
                case "n":       j++; break;
                case "ne": i++; j++; break;
                case "nw": i--;      break;
                case "se": i++;      break;
                case "sw": i--; j--; break;
                case "s":       j--; break;
                default: throw new IllegalStateException();
            }
            int distance;
            if (i * j < 0) {
                distance = Math.abs(i) + Math.abs(j);
            } else {
                distance = Math.max(Math.abs(i), Math.abs(j));
            }
            if (distance > furthest) {
                furthest = distance;
            }
        }

        System.out.println(furthest);
    }
}
