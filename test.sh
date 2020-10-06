assert() {
  expected="$1"
  input="$2"

  ./target/debug/nani-the-facc "$input" > target/debug/tmp.s || exit
  gcc -static -o target/debug/tmp target/debug/tmp.s
  ./target/debug/tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0
assert 42 42
assert 21 '5+20-4'

echo OK