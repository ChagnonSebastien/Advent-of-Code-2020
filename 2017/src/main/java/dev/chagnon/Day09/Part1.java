package dev.chagnon.Day09;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicReference;
import java.util.stream.IntStream;

public class Part1 {
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
        System.out.println(root.getWeight());
    }

    public static class Node {
        private final List<Node> children = new ArrayList<>();
        private final Node parent;
        private final int height;

        public Node(Node parent) {
            this.parent = parent;
            this.height = parent.getHeight() + 1;
        }

        public Node() {
            this.parent = null;
            this.height = 0;
        }


        public Node addChildren() {
            Node child = new Node(this);
            this.children.add(child);
            return child;
        }
        public Node getParent() {
            return parent;
        }

        public int getHeight() {
            return height;
        }

        public int getWeight() {
            return children.stream().map(Node::getWeight).reduce(height, Integer::sum);
        }
    }
}
