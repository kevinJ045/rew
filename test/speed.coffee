
time = Date.now()
smn = 0
for i = 0; i < 10_000_000_000; i++
  smn++

tim2 = Date.now()

rew::io::out.print smn, (tim2 - time) / 1000