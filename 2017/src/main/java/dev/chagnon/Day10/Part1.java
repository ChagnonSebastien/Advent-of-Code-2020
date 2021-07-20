package dev.chagnon.Day10;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;

public class Part1 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(10);
        int KNOT_LENGTH = 256;

        int[] knot = new int[KNOT_LENGTH];
        for (int i = 0; i < KNOT_LENGTH; i++) {
            knot[i] = i;
        }

        List<Integer> lengths = Arrays.asList(lines.get(0).split(",")).stream().map(Integer::parseInt).collect(Collectors.toList());
        int offset = 0;
        int skipSize = 0;

        for (int length : lengths) {
            int[] newKnot = Arrays.copyOf(knot, KNOT_LENGTH);
            int j = offset;
            for (int i = offset + length - 1; i >= offset; i--) {
                newKnot[i % KNOT_LENGTH] = knot[j % KNOT_LENGTH];
                j++;
            }

            knot = newKnot;
            offset = (offset + length + skipSize++) % KNOT_LENGTH;
        }

        System.out.println(knot[0] * knot[1]);
    }
}
