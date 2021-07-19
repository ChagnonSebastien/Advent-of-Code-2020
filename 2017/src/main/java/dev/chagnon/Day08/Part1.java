package dev.chagnon.Day08;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.*;

public class Part1 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(8);
        Registries registries = new Registries();

        for (String line : lines) {
            String[] instructions = line.split(" ");

            int conditionValue = Integer.parseInt(instructions[6]);
            boolean conditionMet = switch (instructions[5]) {
                case "==" -> registries.eq(instructions[4], conditionValue);
                case "!=" -> registries.ne(instructions[4], conditionValue);
                case "<" -> registries.st(instructions[4], conditionValue);
                case "<=" -> registries.ste(instructions[4], conditionValue);
                case ">" -> registries.gt(instructions[4], conditionValue);
                case ">=" -> registries.gte(instructions[4], conditionValue);
                default -> throw new IllegalStateException("Unexpected value: " + instructions[5]);
            };

            if (conditionMet) {
                int equationValue = Integer.parseInt(instructions[2]);
                if (instructions[1].equals("inc")) {
                    registries.increment(instructions[0], equationValue);
                } else {
                    registries.decrement(instructions[0], equationValue);
                }
            }
        }

        System.out.println(registries.largestRegistryValue());
    }

    public static class Registries {
        private final Map<String, Integer> registries = new HashMap<>();

        public void increment(String registry, int amount) {
            registries.put(registry, registries.getOrDefault(registry, 0) + amount);
        }

        public void decrement(String registry, int amount) {
            registries.put(registry, registries.getOrDefault(registry, 0) - amount);
        }

        public boolean eq(String registry, int value) {
            return registries.getOrDefault(registry, 0) == value;
        }

        public boolean ne(String registry, int value) {
            return registries.getOrDefault(registry, 0) != value;
        }

        public boolean st(String registry, int value) {
            return registries.getOrDefault(registry, 0) < value;
        }

        public boolean ste(String registry, int value) {
            return registries.getOrDefault(registry, 0) <= value;
        }

        public boolean gt(String registry, int value) {
            return registries.getOrDefault(registry, 0) > value;
        }

        public boolean gte(String registry, int value) {
            return registries.getOrDefault(registry, 0) >= value;
        }

        public int largestRegistryValue() {
            return registries.values().stream().max(Integer::compare).orElse(0);
        }
    }
}
