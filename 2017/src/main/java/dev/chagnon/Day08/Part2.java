package dev.chagnon.Day08;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.List;

public class Part2 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(8);
        Part1.Registries registries = new Part1.Registries();
        int largestGlobal = 0;

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

            int largestSnapshot = registries.largestRegistryValue();
            if (largestSnapshot > largestGlobal) {
                largestGlobal = largestSnapshot;
            }
        }

        System.out.println(largestGlobal);
    }
}
