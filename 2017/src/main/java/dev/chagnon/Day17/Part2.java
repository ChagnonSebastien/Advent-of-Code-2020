package dev.chagnon.Day17;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.List;

public class Part2 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        int input = Integer.parseInt(Utils.getInput(17).get(0));

        int index = 0;
        int currentAfterZero = 0;
        for (int i = 1; i <= 50_000_000; i++) {
            index = (index + input) % i;
            if (index == 0) {
                currentAfterZero = i;
            }
            index = (index + 1) % (i + 1);
        }

        System.out.println(currentAfterZero);
    }
}
