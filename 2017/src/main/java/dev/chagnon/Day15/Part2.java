package dev.chagnon.Day15;

import dev.chagnon.Utils;

import java.io.IOException;
import java.math.BigInteger;
import java.net.URISyntaxException;
import java.util.List;
import java.util.regex.Pattern;

public class Part2 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(15);
        Pattern pattern = Pattern.compile("[0-9]+");

        BigInteger a = BigInteger.valueOf(Long.parseLong(pattern.matcher(lines.get(0)).results().findAny().get().group()));
        BigInteger b = BigInteger.valueOf(Long.parseLong(pattern.matcher(lines.get(1)).results().findAny().get().group()));

        int matches = 0;
        for (int i = 0; i < 5_000_000; i++) {
            do {
                a = a.multiply(BigInteger.valueOf(16807)).mod(BigInteger.valueOf(2147483647));
            } while (!a.mod(BigInteger.valueOf(4)).equals(BigInteger.ZERO));
            do {
                b = b.multiply(BigInteger.valueOf(48271)).mod(BigInteger.valueOf(2147483647));
            } while (!b.mod(BigInteger.valueOf(8)).equals(BigInteger.ZERO));

            if (a.and(BigInteger.valueOf(0b1111_1111_1111_1111)).xor(b.and(BigInteger.valueOf(0b1111_1111_1111_1111))).equals(BigInteger.ZERO)) {
                matches += 1;
            }
        }

        System.out.println(matches);
    }
}
