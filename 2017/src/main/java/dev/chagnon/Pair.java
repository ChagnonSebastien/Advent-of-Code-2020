package dev.chagnon;

import java.util.Objects;

public class Pair<Key, Value> {
    private Key key;

    private Value value;

    public Pair(Key key, Value value) {
        this.key = key;
        this.value = value;
    }

    public Value getValue() {
        return value;
    }

    public void setValue(Value value) {
        this.value = value;
    }

    public Key getKey() {
        return key;
    }

    public void setKey(Key key) {
        this.key = key;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Pair<?, ?> pair = (Pair<?, ?>) o;
        return Objects.equals(getKey(), pair.getKey()) && Objects.equals(getValue(), pair.getValue());
    }

    @Override
    public int hashCode() {
        return Objects.hash(getKey(), getValue());
    }
}
