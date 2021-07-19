package dev.chagnon.Day09;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.IntStream;

public class Part2 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(9);
        IntStream stream = lines.get(0).chars();

        Node root = new Node();

        final var current = new Object() {
            Node node = root;
            boolean inGarbage = false;
            boolean nextCharEscaped = false;
        };

        stream.forEachOrdered(c -> {
            if (current.inGarbage) {
                if (!current.nextCharEscaped && c == '>') {
                    current.inGarbage = false;
                } else if (!current.nextCharEscaped && c == '!') {
                    current.nextCharEscaped = true;
                } else if (current.nextCharEscaped) {
                    current.nextCharEscaped = false;
                } else {
                    current.node.addToGarbage((char) c);
                }
            } else {
                switch (c) {
                    case '{': current.node = current.node.addChildren(); break;
                    case '}': current.node = current.node.getParent(); break;
                    case '<': current.inGarbage = true; break;
                    case ',': /* ignore */ break;
                    default: throw new IllegalStateException();
                }
            }
        });

        assert current.node == root;
        System.out.println(root.getGarbageLength());
    }

    public static class Node {
        private final List<Node> children = new ArrayList<>();
        private final Node parent;
        private String garbage = "";

        public Node(Node parent) {
            this.parent = parent;
        }

        public Node() {
            this.parent = null;
        }


        public Node addChildren() {
            Node child = new Node(this);
            this.children.add(child);
            return child;
        }
        public Node getParent() {
            return parent;
        }

        public void addToGarbage(char newChar) {
            garbage += newChar;
        }

        public int getGarbageLength() {
            return children.stream().map(Node::getGarbageLength).reduce(garbage.length(), Integer::sum);
        }
    }
}
