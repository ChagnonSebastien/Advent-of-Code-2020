package dev.chagnon.Day02;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.List;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Part1 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(2);
        Pattern pattern = Pattern.compile("[0-9]+");
        int sum = 0;
        for (String row : lines) {
            int min = Integer.MAX_VALUE;
            int max = Integer.MIN_VALUE;
            for (MatchResult matchResult : pattern.matcher(row).results().collect(Collectors.toList())) {
                int parsedElement = Integer.parseInt(matchResult.group());
                if (parsedElement < min) {
                    min = parsedElement;
                }
                if (parsedElement > max) {
                    max = parsedElement;
                }
            }
            sum += (max - min);
        }
        System.out.println(sum);
    }
}
