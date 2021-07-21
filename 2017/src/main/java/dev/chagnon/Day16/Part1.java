package dev.chagnon.Day16;

import dev.chagnon.Pair;
import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.stream.Collectors;

public class Part1 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(16);
        List<Pair<Character, String[]>> instructions = Arrays.stream(lines.get(0).split(","))
                .map(s -> new Pair<>(s.charAt(0), s.substring(1).split("/")))
                .collect(Collectors.toList());

        HashMap<Character, Integer> indexMapping = new HashMap<>();
        char[] programs = new char[16];
        int arrayStart = 0;

        for (int i = 0; i < 16; i++) {
            char c = (char) ('a' + i);
            programs[i] = c;
            indexMapping.put(c, i);
        }

        for (Pair<Character, String[]> instruction : instructions) {
            switch (instruction.getKey()) {
                case 's' -> arrayStart = (arrayStart + (programs.length - Integer.parseInt(instruction.getValue()[0]))) % programs.length;
                case 'x' -> {
                    int a = (Integer.parseInt(instruction.getValue()[0]) + arrayStart) % programs.length;
                    int b = (Integer.parseInt(instruction.getValue()[1]) + arrayStart) % programs.length;
                    char aValue = programs[a];
                    char bValue = programs[b];
                    programs[a] = bValue;
                    programs[b] = aValue;
                    indexMapping.put(aValue, b);
                    indexMapping.put(bValue, a);
                }
                case 'p' -> {
                    char aValue = instruction.getValue()[0].charAt(0);
                    char bValue = instruction.getValue()[1].charAt(0);
                    int a = indexMapping.get(aValue);
                    int b = indexMapping.get(bValue);
                    programs[a] = bValue;
                    programs[b] = aValue;
                    indexMapping.put(aValue, b);
                    indexMapping.put(bValue, a);
                }
                default -> throw new IllegalStateException("Unknown instruction: " + instruction.getKey());
            }
        }

        for (int i = arrayStart; i < arrayStart + programs.length; i++) {
            System.out.print(programs[i % programs.length]);
        }
    }
}
