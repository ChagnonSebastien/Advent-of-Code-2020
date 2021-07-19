package dev.chagnon.Day06;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Part1 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(6);
        Pattern pattern = Pattern.compile("[0-9]+");
        List<Integer> memory = pattern
                .matcher(lines.get(0)).results()
                .map(MatchResult::group)
                .map(Integer::parseInt)
                .collect(Collectors.toList());

        int cycles = 0;
        Set<String> visitedStates = new HashSet<>();

        String snapshot = memorySnapshot(memory);
        do {
            visitedStates.add(snapshot);

            int amountBlocks = memory.stream().reduce(0, Math::max);
            int index = memory.indexOf(amountBlocks);

            memory.set(index, 0);
            do {
                index = ((index + 1) % memory.size());
                memory.set(index, memory.get(index) + 1);
                amountBlocks -= 1;
            } while (amountBlocks > 0);
            cycles += 1;

            snapshot = memorySnapshot(memory);
        } while(!visitedStates.contains(snapshot));

        System.out.println(cycles);
    }

    public static String memorySnapshot(List<Integer> memory) {
        return memory.stream().map(String::valueOf).collect(Collectors.joining("-"));
    }
}
