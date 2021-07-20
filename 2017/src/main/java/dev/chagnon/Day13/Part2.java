package dev.chagnon.Day13;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Part2 {
    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(13);

        Map<Integer, Layer> firewall = new HashMap<>();
        for (String line : lines) {
            String[] data = line.split(": ");
            firewall.put(Integer.valueOf(data[0]), new Layer(Integer.parseInt(data[1])));
        }

        firewall.forEach((delay, layer) -> {
            for (int i = 0; i < delay; i++) {
                layer.move();
            }
        });

        int waitTime = 0;
        while (firewall.values().stream().anyMatch(layer -> layer.getScanner() == 0)) {
            waitTime += 1;
            firewall.values().forEach(Layer::move);
        }

        System.out.println(waitTime);
    }

    public static class Layer {
        private final int range;
        private int scanner;
        private int nextMove = 1;

        public Layer(int range) {
            this.range = range;
        }

        public Layer(int range, int scanner, int nextMove) {
            this.range = range;
            this.scanner = scanner;
            this.nextMove = nextMove;
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

        @Override
        public String toString() {
            return "Layer{" + scanner + "/" + range + '}';
        }

        public Layer copy() {
            return new Layer(range, scanner, nextMove);
        }
    }
}
