package dev.chagnon.Day02;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.List;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Part2 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(2);
        Pattern pattern = Pattern.compile("[0-9]+");
        int sum = 0;

        for (String row : lines) {
            List<Integer> sortedRow = pattern
                    .matcher(row)
                    .results()
                    .map(MatchResult::group)
                    .map(Integer::parseInt)
                    .sorted()
                    .collect(Collectors.toList());

            external:
            for (int i = 0; i < sortedRow.size(); i++) {
                for (int j = i + 1; j < sortedRow.size(); j++) {
                    int smaller = sortedRow.get(i);
                    int larger = sortedRow.get(j);
                    if (larger % smaller == 0) {
                        sum += larger / smaller;
                        break external;
                    }
                }
            }
        }

        System.out.println(sum);
    }
}
