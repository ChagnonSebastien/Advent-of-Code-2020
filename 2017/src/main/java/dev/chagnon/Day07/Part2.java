package dev.chagnon.Day07;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.*;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Part2 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(7);
        Pattern numberPattern = Pattern.compile("[0-9]+");
        Pattern lowercaseStringPattern = Pattern.compile("[a-z]+");

        Map<String, Set<String>> hierarchicalData = new HashMap<>();
        Map<String, Integer> weightData = new HashMap<>();

        for (String line : lines) {
            Deque<String> children = lowercaseStringPattern
                    .matcher(line)
                    .results()
                    .map(MatchResult::group)
                    .collect(Collectors.toCollection(ArrayDeque::new));

            String nodeName = children.pop();
            int nodeWeight = numberPattern
                    .matcher(line)
                    .results()
                    .map(MatchResult::group)
                    .map(Integer::parseInt)
                    .collect(Collectors.toList())
                    .get(0);

            hierarchicalData.put(nodeName, new HashSet<>(children));
            weightData.put(nodeName, nodeWeight);
        }

        Set<String> childrenNodes = hierarchicalData.values().stream().reduce(new HashSet<>(), (a, b) -> {
            Set<String> sum = new HashSet<>();
            sum.addAll(a);
            sum.addAll(b);
            return sum;
        });

        String root = hierarchicalData.keySet().stream().filter(a -> !childrenNodes.contains(a)).iterator().next();

        try {
            balance(hierarchicalData, weightData, root);
        } catch (Exception e) {
            System.out.println(e.getMessage());
        }

    }

    public static int balance(Map<String, Set<String>> hierarchicalData, Map<String, Integer> weightData, String node) throws Exception {
        Map<Integer, Set<String>> childrenWeights = new HashMap<>();
        for (String c : hierarchicalData.get(node)) {
            int w = balance(hierarchicalData, weightData, c);
            Set<String> prev = childrenWeights.getOrDefault(w, new HashSet<>());
            prev.add(c);
            childrenWeights.put(w, prev);
        }
        if (childrenWeights.size() > 1) {
            Map.Entry<Integer, Set<String>> entry = childrenWeights.entrySet().stream().filter(e -> e.getValue().size() == 1).iterator().next();
            int requiredValue = childrenWeights.entrySet().stream().filter(e -> e.getValue().size() != 1).map(Map.Entry::getKey).iterator().next();
            throw new Exception(String.valueOf((requiredValue - entry.getKey()) + weightData.get(entry.getValue().iterator().next())));
        }
        return weightData.get(node) + (hierarchicalData.get(node).size() * (childrenWeights.size() == 0 ? 0 : childrenWeights.keySet().iterator().next()));
    }
}
