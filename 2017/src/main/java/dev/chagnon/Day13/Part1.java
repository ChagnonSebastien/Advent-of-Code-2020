package dev.chagnon.Day13;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Part1 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(13);

        Map<Integer, Layer> firewall = new HashMap<>();
        for (String line : lines) {
            String[] data = line.split(": ");
            firewall.put(Integer.valueOf(data[0]), new Layer(Integer.parseInt(data[1])));
        }

        int toTravel = firewall.keySet().stream().max(Integer::compare).orElse(firewall.size());
        int severity = 0;
        for (int packet = 0; packet <= toTravel; packet++) {
            if (firewall.containsKey(packet)) {
                Layer layer = firewall.get(packet);
                if (layer.getScanner() == 0) {
                    severity += layer.getRange() * packet;
                }
            }

            firewall.values().forEach(Layer::move);
        }

        System.out.println(severity);
    }

    public static class Layer {
        private final int range;
        private int scanner;
        private int nextMove = 1;

        public Layer(int range) {
            this.range = range;
        }

        public void move() {
            scanner += nextMove;
            if (scanner == 0) {
                nextMove = 1;
            }
            if (scanner == range - 1) {
                nextMove = -1;
            }
        }

        public int getScanner() {
            return scanner;
        }

        public int getRange() {
            return range;
        }
    }
}
