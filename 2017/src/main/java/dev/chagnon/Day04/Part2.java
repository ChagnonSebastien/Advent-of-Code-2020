package dev.chagnon.Day04;

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
        List<String> lines = Utils.getInput(4);
        Pattern pattern = Pattern.compile("[a-z]+");
        int validAmount = 0;
        for (String passphrase : lines) {
            List<Map<Character, Integer>> wordsContents = pattern
                    .matcher(passphrase)
                    .results()
                    .map(MatchResult::group)
                    .map(Part2::parseContents)
                    .collect(Collectors.toList());
            if (!hasAnagram(wordsContents)) {
                validAmount += 1;
            }
        }
        System.out.println(validAmount);
    }

    public static boolean hasAnagram(List<Map<Character, Integer>> wordsContents) {
        for (int i = 0; i < wordsContents.size(); i++) {
            for (int j = i + 1; j < wordsContents.size(); j++) {
                if (wordsContents.get(i).equals(wordsContents.get(j))) {
                    return true;
                }
            }
        }
        return false;
    }

    public static Map<Character, Integer> parseContents(String word) {
        HashMap<Character, Integer> contents = new HashMap<>();
        for (char character : word.toCharArray()) {
            contents.put(character, contents.getOrDefault(character, 0));
        }
        return contents;
    }
}
