package dev.chagnon.Day14;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class Part1 {
    public static final int KNOT_LENGTH = 256;

    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(14);
        String keyStringBase = lines.get(0);

        int amountOnes = 0;
        for (int i = 0; i < 128; i++) {
            String keyString = keyStringBase + "-" + i;
            int [] knotHash = knotHash(keyString);
            amountOnes = Arrays.stream(knotHash)
                    .boxed()
                    .map(n -> Integer.toString(n, 2).replaceAll("0", "").length())
                    .reduce(amountOnes, Integer::sum);
        }

        System.out.println(amountOnes);
    }

    public static int[] knotHash(String keyString) {
        int[] knot = new int[KNOT_LENGTH];
        for (int i = 0; i < KNOT_LENGTH; i++) {
            knot[i] = i;
        }

        List<Integer> lengths = new ArrayList<>();
        lengths.addAll(keyString.chars().boxed().collect(Collectors.toList()));
        lengths.addAll(Arrays.asList(17, 31, 73, 47, 23));

        int offset = 0;
        int skipSize = 0;

        for (int round = 0; round < 64; round++) {
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
        }

        int[] denseHash = new int[KNOT_LENGTH / 16];
        for (int i = 0; i < 16; i++) {
            int v = 0;
            for (int j = 0; j < 16 ; j++) {
                v = v ^ knot[i * 16 + j];
            }
            denseHash[i] = v;
        }

        return denseHash;
    }
}
