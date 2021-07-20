package dev.chagnon.Day12;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.*;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Part1 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(12);
        Pattern pattern = Pattern.compile("[0-9]+");

        Map<Integer, Set<Integer>> graph = new HashMap<>();
        for (String line : lines) {
            Deque<Integer> matches = pattern
                    .matcher(line)
                    .results()
                    .map(MatchResult::group)
                    .map(Integer::parseInt)
                    .collect(Collectors.toCollection(ArrayDeque::new));

            Integer element = matches.pop();
            graph.put(element, new HashSet<>(matches));
        }

        List<Set<Integer>> groups = new ArrayList<>();
        for (int node : graph.keySet()) {
            if (groups.stream().anyMatch(g -> g.contains(node))) {
                continue;
            }

            Set<Integer> group = new HashSet<>();
            Stack<Integer> ends = new Stack<>();
            ends.push(node);

            while (!ends.empty()) {
                int end = ends.pop();
                if (!group.contains(end)) {
                    group.add(end);
                    for (int neighbor : graph.get(end)) {
                        ends.push(neighbor);
                    }
                }
            }

            groups.add(group);
        }

        System.out.println(groups.size());
    }
}
