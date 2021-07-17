package dev.chagnon.Day01;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.List;

public class Part2 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(1);
        String captcha = lines.get(0);

        int sum = 0;

        for (int i = 0; i < captcha.length(); i++) {
            char current = captcha.charAt(i);
            int otherSide = (i + (captcha.length() / 2)) % captcha.length();
            if (current == captcha.charAt(otherSide)) {
                sum += Integer.parseInt(String.valueOf(current));
            }
        }

        System.out.println(sum);
    }
}
