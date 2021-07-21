package dev.chagnon.Day16;

import dev.chagnon.Utils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.*;
import java.util.stream.Collectors;

public class Part2 {
    static HashMap<Character, Integer> indexMapping = new HashMap<>();
    static char[] programs = new char[16];
    static byte arrayStart = 0;

    public static void main(String[] args) throws IOException, URISyntaxException {
        List<String> lines = Utils.getInput(16);
        List<Instruction> instructions = Arrays.stream(lines.get(0).split(","))
                .map(s -> switch (s.charAt(0)) {
                    case 's' -> Spin.fromRaw(s.substring(1));
                    case 'x' -> Exchange.fromRaw(s.substring(1));
                    case 'p' -> Partner.fromRaw(s.substring(1));
                    default -> throw new IllegalStateException("Unexpected value: " + s.charAt(0)); })
                .collect(Collectors.toList());

        for (int i = 0; i < 16; i++) {
            char c = (char) ('a' + i);
            programs[i] = c;
            indexMapping.put(c, i);
        }

        HashMap<State, Integer> visited = new HashMap<>();
        for (int i = 0; i < 1_000_000_000; i++) {
            for (Instruction instruction : instructions) {
                instruction.apply();
            }
            State state = new State(arrayStart, Arrays.copyOf(programs, programs.length));
            if (visited.containsKey(state)) {
                double jump = i - visited.get(state);
                i += Math.floor((1_000_000_000 - i) / jump) * jump;
            } else {
                visited.put(state, i);
            }
        }

        for (int i = arrayStart; i < arrayStart + programs.length; i++) {
            System.out.print(programs[i % programs.length]);
        }
    }

    interface Instruction {
        void apply();
    }

    record Spin(int amount) implements Instruction {

        @Override
        public void apply() {
            arrayStart = (byte) ((arrayStart + (programs.length - amount)) % programs.length);
        }

        static Spin fromRaw(String raw) {
            return new Spin(Integer.parseInt(raw));
        }
    }

    record Exchange(int rawA, int rawB) implements Instruction {

        @Override
        public void apply() {
            int a = (rawA + arrayStart) % programs.length;
            int b = (rawB + arrayStart) % programs.length;
            char aValue = programs[a];
            char bValue = programs[b];
            programs[a] = bValue;
            programs[b] = aValue;
            indexMapping.put(aValue, b);
            indexMapping.put(bValue, a);
        }

        static Exchange fromRaw(String raw) {
            String[] t =  raw.split("/");
            return new Exchange(Integer.parseInt(t[0]), Integer.parseInt(t[1]));
        }
    }

    record Partner(char aValue, char bValue) implements Instruction {

        @Override
        public void apply() {
            int a = indexMapping.get(aValue);
            int b = indexMapping.get(bValue);
            programs[a] = bValue;
            programs[b] = aValue;
            indexMapping.put(aValue, b);
            indexMapping.put(bValue, a);
        }

        static Partner fromRaw(String raw) {
            String[] t =  raw.split("/");
            return new Partner(t[0].charAt(0), t[1].charAt(0));
        }
    }

    record State(byte arrayStart, char[] programs) {
        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            State state = (State) o;
            return arrayStart == state.arrayStart && Arrays.equals(programs, state.programs);
        }

        @Override
        public int hashCode() {
            int result = Objects.hash(arrayStart);
            result = 31 * result + Arrays.hashCode(programs);
            return result;
        }
    }
}
