package dev.chagnon.Day17;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.List;

public class Part1 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(17);
        int input = Integer.parseInt(lines.get(0));

        Link current = new Link(0);
        for (int i = 1; i <= 2017; i++) {
            for (int j = 0; j < input; j++) {
                current = current.after;
            }
            current = new Link(i, current, current.after);
        }

        System.out.println(current.after.value);

    }

    static class Link {
        int value;
        Link before;
        Link after;

        Link(int value, Link before, Link after) {
            this.value = value;
            this.before = before;
            this.after = after;
            before.after = this;
            after.before = this;
        }

        Link(int value) {
            this.value = value;
            this.before = this;
            this.after = this;
        }
    }
}
