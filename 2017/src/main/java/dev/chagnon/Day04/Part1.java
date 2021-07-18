package dev.chagnon.Day04;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.*;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Part1 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(4);
        Pattern pattern = Pattern.compile("[a-z]+");
        int validAmount = 0;
        for (String passphrase : lines) {
            List<String> words = pattern
                    .matcher(passphrase)
                    .results()
                    .map(MatchResult::group)
                    .collect(Collectors.toList());
            if (!hasDuplicate(words)) {
                validAmount += 1;
            }
        }
        System.out.println(validAmount);
    }

    public static boolean hasDuplicate(List<String> words) {
        for (int i = 0; i < words.size(); i++) {
            for (int j = i + 1; j < words.size(); j++) {
                if (words.get(i).equals(words.get(j))) {
                    return true;
                }
            }
        }
        return false;
    }
}
