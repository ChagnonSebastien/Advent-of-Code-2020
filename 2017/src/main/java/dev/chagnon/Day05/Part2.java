package dev.chagnon.Day05;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Part2 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(5);
        List<Integer> instructions = lines.stream().map(Integer::parseInt).collect(Collectors.toList());
        int offset = 0;
        int steps = 0;

        while (offset >= 0 && offset < instructions.size()) {
            int instruction = instructions.get(offset);
            instructions.set(offset, instruction >= 3 ? instruction - 1 : instruction + 1);
            offset += instruction;
            steps += 1;
        }

        System.out.println(steps);
    }
}
