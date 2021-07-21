package dev.chagnon;

import java.util.HashSet;
import java.util.Objects;

public record Coordinate(int x, int y) {

    public int getX() {
        return x;
    }

    public int getY() {
        return y;
    }

    public static Coordinate from(int x, int y) {
        return new Coordinate(x, y);
    }

    public HashSet<Coordinate> getNeighbors(int minX, int minY, int maxX, int maxY) {
        HashSet<Coordinate> neighbors = new HashSet<>();
        if (x > minX) {
            neighbors.add(Coordinate.from(x - 1, y));
        }
        if (y > minY) {
            neighbors.add(Coordinate.from(x, y - 1));
        }
        if (x < maxX) {
            neighbors.add(Coordinate.from(x + 1, y));
        }
        if (y < maxY) {
            neighbors.add(Coordinate.from(x, y + 1));
        }
        return neighbors;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Coordinate that = (Coordinate) o;
        return getX() == that.getX() && getY() == that.getY();
    }

    @Override
    public int hashCode() {
        return Objects.hash(getX(), getY());
    }

    @Override
    public String toString() {
        return "Coordinate{" +
                "x=" + x +
                ", y=" + y +
                '}';
    }
}
