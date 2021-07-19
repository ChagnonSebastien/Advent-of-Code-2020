package dev.chagnon.Day07;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.*;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Part1 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(7);
        Pattern numberPattern = Pattern.compile("[0-9]+");
        Pattern lowercaseStringPattern = Pattern.compile("[a-z]+");

        Map<String, Set<String>> data = new HashMap<>();

        for (String line : lines) {

            Deque<String> children = lowercaseStringPattern
                    .matcher(line)
                    .results()
                    .map(MatchResult::group)
                    .collect(Collectors.toCollection(ArrayDeque::new));

            String nodeName = children.pop();
            data.put(nodeName, new HashSet<>(children));
        }

        HashMap<String, Node> nodes = new HashMap<>();
        for (Map.Entry<String, Set<String>> element : data.entrySet()) {
            Set<Node> children = element
                    .getValue()
                    .stream()
                    .map(childName -> {
                        Node child;
                        if (nodes.containsKey(childName)) {
                            child = nodes.get(childName);
                        } else {
                            child = new Node(childName);
                            nodes.put(childName, child);
                        }
                        return child;
                    })
                    .collect(Collectors.toSet());

            Node node = nodes.get(element.getKey());
            if (node == null) {
                node = new Node(element.getKey(), 0, children);
                nodes.put(element.getKey(), node);
            } else {
                node.setChildren(children);
            }
            Node finalNode = node;
            children.forEach(child -> child.setParent(finalNode));
        }

        Node root = nodes.values().iterator().next();
        while (root.getParent() != null) {
            root = root.getParent();
        }

        System.out.println(root.getName());
    }

    public static class Node<T extends Node> {
        private String name;
        private int weight;
        private Node parent;
        private Set<T> children;

        public Node(String name, int weight, Set<T> children) {
            this.name = name;
            this.weight = weight;
            this.children = children;
        }

        public Node(String name) {
            this.name = name;
            this.children = new HashSet<>();
        }

        public String getName() {
            return name;
        }

        public int getWeight() {
            return weight;
        }

        public void setWeight(int weight) {
            this.weight = weight;
        }

        public Node getParent() {
            return parent;
        }

        public void setParent(Node parent) {
            this.parent = parent;
        }

        public Set<T> getChildren() {
            return children;
        }

        public void setChildren(Set<T> children) {
            this.children = children;
        }

        @Override
        public String toString() {
            return "Node{" +
                    "name='" + name + '\'' +
                    ", weight=" + weight +
                    '}';
        }
    }
}
