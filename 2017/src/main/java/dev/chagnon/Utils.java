package dev.chagnon;

import java.io.FileNotFoundException;
import java.io.IOException;
import java.net.URISyntaxException;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;

public class Utils {
    public static List<String> getInput(int day) throws IOException, URISyntaxException {
        if (day < 1 || day > 25) {
            throw new IllegalArgumentException("The day must be between 1 and 25.");
        }

        String fileName = String.format("input-%s%d", day < 10 ? "0" : "", day);

        ClassLoader classLoader = Utils.class.getClassLoader();
        URL resource = classLoader.getResource(fileName);
        if (resource == null) {
            throw new FileNotFoundException();
        }

        Path inputFilePath = Paths.get(resource.toURI());
        return Files.readAllLines(inputFilePath, StandardCharsets.UTF_8);
    }
}
